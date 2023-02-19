#![allow(unused_imports)]

mod builders;

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quick_xml::reader::Reader;
use quote::{quote, ToTokens};
use syn::{parse::Parse, parse_macro_input, DeriveInput, Expr, ExprLit, Ident, Lit, LitStr};

use builders::*;

#[proc_macro]
pub fn load_css(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as TokenTree);

    let expanded = quote! {
        {
            let provider = CssProvider::new();
            provider.load_from_data(include_str!(#file));
            StyleContext::add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
struct Field {
    name: String,
    r#type: String,
}

struct Object {
    name: String,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let name = syn::parse_str::<Ident>(&self.name).unwrap();
        let r#type = syn::parse_str::<Ident>(&self.r#type).unwrap();
        tokens.extend(quote! {
            #name: #r#type
        });
    }
}

impl ToTokens for Object {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = syn::parse_str::<Ident>(&self.name).unwrap();
        let name = &self.name;
        tokens.extend(quote! {
            #ident: builder.object(#name).unwrap()
        });
    }
}

#[proc_macro]
pub fn parse_ui(input: TokenStream) -> TokenStream {
    let file = parse_macro_input!(input as LitStr).value();

    // XML Reader
    let xml = std::fs::read_to_string(&file).expect("File does not exist");
    let mut reader = Reader::from_str(&xml);
    reader.trim_text(true);

    // (id, type)
    let mut objects = vec![];
    let mut variables = vec![];

    // Read all the tags with the "class" and "id" property
    loop {
        match reader.read_event() {
            Ok(event) => match event {
                quick_xml::events::Event::Start(ref e) => {
                    if let Some(class) =
                        e.attributes().find(|a| a.as_ref().unwrap().key.0 == b"class")
                    {
                        if let Some(id) = e.attributes().find(|a| a.as_ref().unwrap().key.0 == b"id")
                        {
                            let class = class.unwrap();
                            let id = id.unwrap();
                            let class = std::str::from_utf8(&class.value).unwrap();
                            let id = std::str::from_utf8(&id.value).unwrap();
                            let class = class[3..].to_owned(); // Remove initial Gtk
                            let id = id.to_owned();

                            objects.push(Field {
                                name: (&id).to_owned(),
                                r#type: class,
                            });

                            variables.push(Object {
                                name: (&id).to_owned()
                            })
                        }
                    }
                }
                quick_xml::events::Event::Eof => break,
                _ => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
        }
    }

    let fields = objects.into_iter();
    let variables = variables.into_iter();

    let expanded = quote! {
        {
            let xml = include_str!(#file);
            let builder = Builder::from_string(xml);

            struct Anon {
                #(
                    #fields,
                )*
            };

            Anon {
                #(
                    #variables,
                )*
            }
        }

    };

    TokenStream::from(expanded)
}

make_macro! {about_dialog, AboutDialog}
make_macro! {action_bar, ActionBar}
make_macro! {adjustment, Adjustment}
make_macro! {alert_dialog, AlertDialog}
make_macro! {alternative_trigger, AlternativeTrigger}
make_macro! {app_chooser_button, AppChooserButton}
make_macro! {app_chooser_widget, AppChooserWidget}
make_macro! {application, Application}
make_macro! {application_window, ApplicationWindow}
make_macro! {aspect_frame, AspectFrame}
make_macro! {assistant, Assistant}
make_macro! {bookmark_list, BookmarkList}
make_macro! {bool_filter, BoolFilter}
make_macro! {border, Border}
make_macro! {gtk_box, GTKBox}
make_macro! {box_layout, BoxLayout}
make_macro! {button, Button}
make_macro! {calendar, Calendar}
make_macro! {cell_area_box, CellAreaBox}
make_macro! {cell_renderer_accel, CellRendererAccel}
make_macro! {cell_renderer_combo, CellRendererCombo}
make_macro! {cell_renderer_pixbuf, CellRendererPixbuf}
make_macro! {cell_renderer_progress, CellRendererProgress}
make_macro! {cell_renderer_spin, CellRendererSpin}
make_macro! {cell_renderer_spinner, CellRendererSpinner}
make_macro! {cell_renderer_text, CellRendererText}
make_macro! {cell_renderer_toggle, CellRendererToggle}
make_macro! {cell_view, CellView}
make_macro! {center_box, CenterBox}
make_macro! {check_button, CheckButton}
make_macro! {color_button, ColorButton}
make_macro! {color_chooser_dialog, ColorChooserDialog}
make_macro! {color_chooser_widget, ColorChooserWidget}
make_macro! {color_dialog, ColorDialog}
make_macro! {color_dialog_button, ColorDialogButton}
make_macro! {column_view, ColumnView}
make_macro! {column_view_column, ColumnViewColumn}
make_macro! {combo_box, ComboBox}
make_macro! {combo_box_text, ComboBoxText}
make_macro! {constraint, Constraint}
make_macro! {constraint_guide, ConstraintGuide}
make_macro! {dialog, Dialog}
make_macro! {directory_list, DirectoryList}
make_macro! {drag_source, DragSource}
make_macro! {drawing_area, DrawingArea}
make_macro! {drop_controller_motion, DropControllerMotion}
make_macro! {drop_down, DropDown}
make_macro! {drop_target_async, DropTargetAsync}
make_macro! {drop_target, DropTarget}
make_macro! {editable_label, EditableLabel}
make_macro! {emoji_chooser, EmojiChooser}
make_macro! {entry_buffer, EntryBuffer}
make_macro! {entry, Entry}
make_macro! {entry_completion, EntryCompletion}
make_macro! {event_controller_focus, EventControllerFocus}
make_macro! {event_controller_key, EventControllerKey}
make_macro! {event_controller_legacy, EventControllerLegacy}
make_macro! {event_controller_motion, EventControllerMotion}
make_macro! {event_controller_scroll, EventControllerScroll}
make_macro! {expander, Expander}
make_macro! {file_chooser_dialog, FileChooserDialog}
make_macro! {file_chooser_native, FileChooserNative}
make_macro! {file_chooser_widget, FileChooserWidget}
make_macro! {file_dialog, FileDialog}
make_macro! {filter_list_model, FilterListModel}
make_macro! {fixed, Fixed}
make_macro! {flow_box, FlowBox}
make_macro! {flow_box_child, FlowBoxChild}
make_macro! {font_button, FontButton}
make_macro! {font_chooser_dialog, FontChooserDialog}
make_macro! {font_chooser_widget, FontChooserWidget}
make_macro! {font_dialog, FontDialog}
make_macro! {font_dialog_button, FontDialogButton}
make_macro! {frame, Frame}
make_macro! {gl_area, GLArea}
make_macro! {gesture_click, GestureClick}
make_macro! {gesture_drag, GestureDrag}
make_macro! {gesture_long_press, GestureLongPress}
make_macro! {gesture_pan, GesturePan}
make_macro! {gesture_rotate, GestureRotate}
make_macro! {gesture_stylus, GestureStylus}
make_macro! {gesture_swipe, GestureSwipe}
make_macro! {gesture_zoom, GestureZoom}
make_macro! {grid, Grid}
make_macro! {grid_layout, GridLayout}
make_macro! {grid_view, GridView}
make_macro! {header_bar, HeaderBar}
make_macro! {im_context_simple, IMContextSimple}
make_macro! {im_multicontext, IMMulticontext}
make_macro! {icon_theme, IconTheme}
make_macro! {icon_view, IconView}
make_macro! {image, Image}
make_macro! {info_bar, InfoBar}
make_macro! {inscription, Inscription}
make_macro! {label, Label}
make_macro! {level_bar, LevelBar}
make_macro! {link_button, LinkButton}
make_macro! {list_box, ListBox}
make_macro! {list_box_row, ListBoxRow}
make_macro! {list_view, ListView}
make_macro! {lock_button, LockButton}
make_macro! {media_controls, MediaControls}
make_macro! {menu_button, MenuButton}
make_macro! {message_dialog, MessageDialog}
make_macro! {mount_operation, MountOperation}
make_macro! {notebook, Notebook}
make_macro! {numeric_sorter, NumericSorter}
make_macro! {overlay, Overlay}
make_macro! {pad_controller, PadController}
make_macro! {page_setup_unix_dialog_build, PageSetupUnixDialogBuild}
make_macro! {paned, Paned}
make_macro! {param_spec_expression, ParamSpecExpression}
make_macro! {password_entry_buffer, PasswordEntryBuffer}
make_macro! {password_entry, PasswordEntry}
make_macro! {picture, Picture}
make_macro! {popover, Popover}
make_macro! {popover_menu_bar, PopoverMenuBar}
make_macro! {popover_menu, PopoverMenu}
make_macro! {print_operation, PrintOperation}
make_macro! {print_unix_dialog_build, PrintUnixDialogBuild}
make_macro! {progress_bar, ProgressBar}
make_macro! {revealer, Revealer}
make_macro! {scale, Scale}
make_macro! {scale_button, ScaleButton}
make_macro! {scrollbar, Scrollbar}
make_macro! {scrolled_window, ScrolledWindow}
make_macro! {search_bar, SearchBar}
make_macro! {search_entry, SearchEntry}
make_macro! {separator, Separator}
make_macro! {settings, Settings}
make_macro! {shortcut, Shortcut}
make_macro! {shortcut_label, ShortcutLabel}
make_macro! {shortcuts_group, ShortcutsGroup}
make_macro! {shortcuts_section, ShortcutsSection}
make_macro! {shortcuts_shortcut, ShortcutsShortcut}
make_macro! {shortcuts_window, ShortcutsWindow}
make_macro! {single_selection, SingleSelection}
make_macro! {slice_list_model, SliceListModel}
make_macro! {sort_list_model, SortListModel}
make_macro! {spin_button, SpinButton}
make_macro! {spinner, Spinner}
make_macro! {stack, Stack}
make_macro! {stack_sidebar, StackSidebar}
make_macro! {stack_switcher, StackSwitcher}
make_macro! {statusbar, Statusbar}
make_macro! {string_filter, StringFilter}
make_macro! {string_sorter, StringSorter}
make_macro! {switch, Switch}
make_macro! {text_buffer, TextBuffer}
make_macro! {text, Text}
make_macro! {text_mark, TextMark}
make_macro! {text_tag, TextTag}
make_macro! {text_view, TextView}
make_macro! {toggle_button, ToggleButton}
make_macro! {tree_expander, TreeExpander}
make_macro! {tree_view, TreeView}
make_macro! {tree_view_column, TreeViewColumn}
make_macro! {uri_launcher, UriLauncher}
make_macro! {video, Video}
make_macro! {viewport, Viewport}
make_macro! {volume_button, VolumeButton}
make_macro! {window, Window}
make_macro! {window_controls, WindowControls}
make_macro! {window_handle, WindowHandle}
