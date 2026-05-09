use crate::browser_discovery::get_browser_name_from_path;
use log::{error, warn};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

const BROWSER_CHOOSER_TITLE: &str = "Choose Browser";
const BROWSER_CHOOSER_PROMPT: &str = "Select a browser to open:";
const BROWSER_CHOOSER_REMEMBER_CHOICE: &str = "Remember this choice for this site";
const BROWSER_CHOOSER_CANCEL: &str = "Cancel";

slint::slint! {
    import { Button, CheckBox, VerticalBox } from "std-widgets.slint";

    export component BrowserChooserDialog inherits Window {
        in property <string> window_title;
        in property <string> prompt_text;
        in property <string> remember_choice_text;
        in property <string> cancel_text;
        in property <string> url;
        in property <[string]> browsers;
        in-out property <bool> remember_choice: false;
        callback browser_selected(int, bool);
        callback cancel();

        width: 520px;
        height: 380px;
        title: root.window_title;

        VerticalBox {
            padding: 16px;
            spacing: 12px;

            Text {
                text: root.prompt_text;
                font-size: 18px;
                wrap: word-wrap;
            }

            Text {
                text: root.url;
                color: #666666;
                wrap: word-wrap;
            }

            VerticalBox {
                spacing: 8px;

                for browser[i] in root.browsers : Button {
                    text: browser;
                    clicked => { root.browser_selected(i, root.remember_choice); }
                }
            }

            CheckBox {
                text: root.remember_choice_text;
                checked <=> root.remember_choice;
            }

            Button {
                text: root.cancel_text;
                clicked => { root.cancel(); }
            }
        }
    }
}

pub enum GuiChooserOutcome {
    Selected {
        browser_path: String,
        save_rule: bool,
    },
    Cancelled,
    Unavailable,
}

pub fn prompt_browser_selection_slint(url: &str, browsers: &[String]) -> GuiChooserOutcome {
    let dialog = match BrowserChooserDialog::new() {
        Ok(dialog) => dialog,
        Err(e) => {
            warn!("Slint chooser could not be started: {}", e);
            return GuiChooserOutcome::Unavailable;
        }
    };

    dialog.set_window_title(SharedString::from(BROWSER_CHOOSER_TITLE));
    dialog.set_prompt_text(SharedString::from(BROWSER_CHOOSER_PROMPT));
    dialog.set_remember_choice_text(SharedString::from(BROWSER_CHOOSER_REMEMBER_CHOICE));
    dialog.set_cancel_text(SharedString::from(BROWSER_CHOOSER_CANCEL));

    let browser_names: Vec<SharedString> = browsers
        .iter()
        .map(|browser_path| SharedString::from(browser_display_name(browser_path, browsers)))
        .collect();

    dialog.set_url(SharedString::from(url));
    dialog.set_browsers(ModelRc::new(VecModel::from(browser_names)));

    let browser_paths = browsers.to_vec();
    let result = std::rc::Rc::new(std::cell::RefCell::new(None::<GuiChooserOutcome>));

    {
        let result = std::rc::Rc::clone(&result);
        let weak = dialog.as_weak();
        let browser_paths = browser_paths.clone();

        dialog.on_browser_selected(move |index, save_rule| {
            let browser_index = index as usize;
            let browser_path = match browser_paths.get(browser_index) {
                Some(path) => path.clone(),
                None => {
                    error!("Invalid browser index selected: {}", index);
                    if let Some(dialog) = weak.upgrade() {
                        let _ = dialog.hide();
                    }
                    return;
                }
            };

            *result.borrow_mut() = Some(GuiChooserOutcome::Selected {
                browser_path,
                save_rule,
            });

            if let Some(dialog) = weak.upgrade() {
                let _ = dialog.hide();
            }
        });
    }

    {
        let result = std::rc::Rc::clone(&result);
        let weak = dialog.as_weak();

        dialog.on_cancel(move || {
            *result.borrow_mut() = Some(GuiChooserOutcome::Cancelled);

            if let Some(dialog) = weak.upgrade() {
                let _ = dialog.hide();
            }
        });
    }

    if let Err(e) = dialog.run() {
        warn!("Slint chooser failed to run: {}", e);
        return GuiChooserOutcome::Unavailable;
    }

    result
        .borrow_mut()
        .take()
        .unwrap_or(GuiChooserOutcome::Cancelled)
}

fn browser_display_name(browser_path: &str, browsers: &[String]) -> String {
    let browser_name = get_browser_name_from_path(browser_path);
    let duplicate_count = browsers
        .iter()
        .filter(|candidate_path| get_browser_name_from_path(candidate_path) == browser_name)
        .count();

    if duplicate_count > 1 {
        format!("{} ({})", browser_name, browser_path)
    } else {
        browser_name
    }
}
