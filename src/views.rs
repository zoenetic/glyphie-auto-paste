use cosmic::iced::widget::scrollable::{Direction, Scrollbar};
use cosmic::iced::{Alignment, Length};
use cosmic::widget::{self, button, container, scrollable};
use cosmic::Element;

use crate::emoji_data::{emoji_data_loaded, EmojiCategory, GenderFilter, SkinTone, EMOJIS};
use crate::fl;
use crate::paste::PasteShortcut;
use crate::theme::{
    accent_tint_button, chip_button, text_muted, CATEGORY_BUTTON_H, CATEGORY_ICON_SIZE,
    CHIP_EMOJI_SIZE, CHIP_HEIGHT, CHIP_PADDING_H, CHIP_PADDING_V, EMOJI_BUTTON_SIZE, EMOJI_FONT,
    FONT_SIZE_EMOJI, FONT_SIZE_EMPTY_ICON, FONT_SIZE_MD, FONT_SIZE_SM, FONT_SIZE_XS, GRID_SPACING,
    SKIN_TONE_CIRCLE_SIZE, SKIN_TONE_RING_GAP, SKIN_TONE_RING_WIDTH, SPACING_MD, SPACING_SM,
    SPACING_XS, SUBCATEGORY_HEADER_HEIGHT, TRAY_SCROLLABLE_HEIGHT,
};
use crate::{
    category_icon, CosmicEmojiPicker, Message, EMOJI_SCROLLABLE_ID, ICON_CARET_DOWN, ICON_CARET_UP,
    ICON_COPY, TRAY_SCROLLABLE_ID,
};

impl CosmicEmojiPicker {
    pub(crate) fn view_category_bar(&self) -> Element<'_, Message> {
        let mut row = widget::Row::new().spacing(0);

        for cat in EmojiCategory::all() {
            let is_selected = *cat == self.selected_category
                && self.search_query.is_empty()
                && self.pending_search.is_none();
            let icon = category_icon(*cat);
            let btn = button::custom(
                container(widget::icon(icon).size(CATEGORY_ICON_SIZE))
                    .width(Length::Fill)
                    .height(Length::Fixed(CATEGORY_BUTTON_H))
                    .align_x(cosmic::iced::alignment::Horizontal::Center)
                    .align_y(cosmic::iced::alignment::Vertical::Center),
            )
            .on_press(Message::CategorySelected(*cat))
            .padding(0)
            .width(Length::Fill)
            .name(cat.name())
            .class(if is_selected {
                cosmic::theme::Button::Suggested
            } else {
                cosmic::theme::Button::Text
            });

            let btn_with_tooltip = widget::tooltip(
                btn,
                widget::text(cat.name()),
                widget::tooltip::Position::Bottom,
            );

            row = row.push(btn_with_tooltip);
        }

        container(row.align_y(Alignment::Center))
            .padding([SPACING_SM, 0.0])
            .width(Length::Fill)
            .into()
    }

    pub(crate) fn view_skin_tone_bar(&self, emoji_count: usize) -> Element<'_, Message> {
        let search_label;
        let category_name: &str = if self.search_query.is_empty() {
            self.selected_category.name()
        } else {
            search_label = fl!("search-label");
            &search_label
        };

        let chevron_icon = if self.show_preferences {
            widget::icon::from_svg_bytes(ICON_CARET_UP).symbolic(true)
        } else {
            widget::icon::from_svg_bytes(ICON_CARET_DOWN).symbolic(true)
        };
        let chevron_btn = widget::tooltip(
            button::icon(chevron_icon)
                .on_press(Message::TogglePreferences)
                .padding(SPACING_XS)
                .class(cosmic::theme::Button::Text),
            widget::text(fl!("preferences")),
            widget::tooltip::Position::Bottom,
        );

        let row = widget::Row::new()
            .push(
                widget::text(format!("{category_name} · {emoji_count}"))
                    .size(FONT_SIZE_SM)
                    .class(text_muted()),
            )
            .push(widget::space::horizontal())
            .push(chevron_btn)
            .spacing(SPACING_SM)
            .align_y(Alignment::Center);

        let header = container(row)
            .padding([SPACING_XS, SPACING_SM])
            .width(Length::Fill);

        let mut col = widget::Column::new().push(header);

        if self.show_preferences {
            col = col.push(self.view_preferences_popover());
        }

        col = col.push(widget::divider::horizontal::default());

        container(col).width(Length::Fill).into()
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn view_preferences_popover(&self) -> Element<'_, Message> {
        let total_size = SKIN_TONE_CIRCLE_SIZE + (SKIN_TONE_RING_GAP + SKIN_TONE_RING_WIDTH) * 2.0;

        let mut skin_row = widget::Row::new().spacing(SPACING_SM);
        for &tone in SkinTone::all() {
            let (r, g, b) = tone.color();
            let color = cosmic::iced::Color::from_rgb(r, g, b);
            let is_selected = tone == self.config.skin_tone;
            let tone_label = match tone {
                SkinTone::Default => fl!("skin-tone-default"),
                SkinTone::Light => fl!("skin-tone-light"),
                SkinTone::MediumLight => fl!("skin-tone-medium-light"),
                SkinTone::Medium => fl!("skin-tone-medium"),
                SkinTone::MediumDark => fl!("skin-tone-medium-dark"),
                SkinTone::Dark => fl!("skin-tone-dark"),
            };

            let dot = button::custom(
                container(
                    container(
                        widget::Space::new()
                            .width(SKIN_TONE_CIRCLE_SIZE)
                            .height(SKIN_TONE_CIRCLE_SIZE),
                    )
                    .class(cosmic::theme::Container::custom(move |_| {
                        cosmic::iced::widget::container::Style {
                            background: Some(cosmic::iced::Background::Color(color)),
                            border: cosmic::iced::Border {
                                radius: (SKIN_TONE_CIRCLE_SIZE / 2.0).into(),
                                width: 0.0,
                                color: cosmic::iced::Color::TRANSPARENT,
                            },
                            ..Default::default()
                        }
                    })),
                )
                .width(Length::Fixed(total_size))
                .height(Length::Fixed(total_size))
                .align_x(cosmic::iced::alignment::Horizontal::Center)
                .align_y(cosmic::iced::alignment::Vertical::Center)
                .class(cosmic::theme::Container::custom(move |theme| {
                    let accent = theme.cosmic().accent_color();
                    cosmic::iced::widget::container::Style {
                        background: None,
                        border: cosmic::iced::Border {
                            radius: (total_size / 2.0).into(),
                            width: if is_selected {
                                SKIN_TONE_RING_WIDTH
                            } else {
                                0.0
                            },
                            color: cosmic::iced::Color::from_rgb(
                                accent.red,
                                accent.green,
                                accent.blue,
                            ),
                        },
                        ..Default::default()
                    }
                })),
            )
            .on_press(Message::SelectSkinTone(tone))
            .padding(0)
            .name(tone_label.clone())
            .class(cosmic::theme::Button::Transparent);

            skin_row = skin_row.push(widget::tooltip(
                dot,
                widget::text(tone_label),
                widget::tooltip::Position::Bottom,
            ));
        }

        let mut gender_row = widget::Row::new().spacing(SPACING_XS);
        for (filter, label) in GenderFilter::all_with_labels() {
            let is_selected = *filter == self.config.gender_filter;

            let text_class = if is_selected {
                cosmic::theme::Text::Accent
            } else {
                cosmic::theme::Text::Default
            };

            let btn = button::custom(widget::text(label).size(FONT_SIZE_SM).class(text_class))
                .on_press(Message::SelectGenderFilter(*filter))
                .padding([SPACING_XS, SPACING_SM])
                .name(label.as_str())
                .class(if is_selected {
                    accent_tint_button()
                } else {
                    cosmic::theme::Button::Text
                });

            gender_row = gender_row.push(btn);
        }

        container(
            widget::Column::new()
                .push(
                    widget::Row::new()
                        .push(
                            widget::text(fl!("skin-tone"))
                                .size(FONT_SIZE_XS)
                                .class(text_muted()),
                        )
                        .push(widget::space::horizontal())
                        .push(skin_row)
                        .align_y(Alignment::Center),
                )
                .push(
                    widget::Row::new()
                        .push(
                            widget::text(fl!("gender"))
                                .size(FONT_SIZE_XS)
                                .class(text_muted()),
                        )
                        .push(widget::space::horizontal())
                        .push(gender_row)
                        .align_y(Alignment::Center),
                )
                .spacing(SPACING_SM),
        )
        .padding(SPACING_SM)
        .width(Length::Fill)
        .class(cosmic::theme::Container::Card)
        .into()
    }

    pub(crate) fn view_settings_menu(&self) -> Element<'_, Message> {
        let subcategory_toggle =
            widget::toggler(self.config.show_subcategories).on_toggle(Message::ToggleSubcategories);

        let auto_paste_toggle =
            widget::toggler(self.config.auto_paste).on_toggle(Message::ToggleAutoPaste);
        let paste_shortcut = widget::dropdown(
            PasteShortcut::labels(),
            Some(self.config.paste_shortcut.position()),
            |i| Message::SelectPasteShortcut(PasteShortcut::ALL[i]),
        );

        let settings_section = widget::settings::section()
            .title(fl!("settings"))
            .add(widget::settings::item(
                fl!("show-subcategories"),
                subcategory_toggle,
            ))
            .add(widget::settings::item(fl!("auto-paste"), auto_paste_toggle))
            .add(widget::settings::item(
                fl!("paste-shortcut"),
                paste_shortcut,
            ));

        let about_section =
            widget::settings::section()
                .title(fl!("information"))
                .add(widget::settings::item(
                    fl!("version"),
                    widget::text(env!("CARGO_PKG_VERSION")),
                ));

        let content =
            widget::settings::view_column(vec![settings_section.into(), about_section.into()]);

        scrollable(container(content).padding(SPACING_XS).width(Length::Fill))
            .height(Length::Fill)
            .into()
    }

    #[allow(clippy::too_many_lines)]
    pub(crate) fn view_selection_tray(&self) -> Option<Element<'_, Message>> {
        if !self.multiselect_mode && self.selected_indices.is_empty() {
            return None;
        }

        let count = self.selected_indices.len();

        // --- Row 1: scrollable emoji pills (or empty-state hint) ---
        let chips_row: Element<'_, Message> = if count == 0 {
            container(
                widget::text(fl!("click-to-select"))
                    .size(FONT_SIZE_SM)
                    .class(text_muted()),
            )
            .height(Length::Fixed(TRAY_SCROLLABLE_HEIGHT))
            .align_y(cosmic::iced::alignment::Vertical::Center)
            .into()
        } else {
            let mut row = widget::Row::new().spacing(SPACING_XS);

            for &idx in &self.selected_indices {
                let display_emoji = self.get_display_emoji(idx);
                let remove_label = fl!("remove-emoji", emoji = display_emoji.to_string());
                let chip = widget::tooltip(
                    button::custom(
                        container(
                            widget::text(display_emoji)
                                .size(CHIP_EMOJI_SIZE)
                                .font(EMOJI_FONT),
                        )
                        .align_x(cosmic::iced::alignment::Horizontal::Center)
                        .align_y(cosmic::iced::alignment::Vertical::Center),
                    )
                    .on_press(Message::RemoveFromSelection(idx))
                    .padding([CHIP_PADDING_V, CHIP_PADDING_H])
                    .height(Length::Fixed(CHIP_HEIGHT))
                    .name(remove_label.clone())
                    .class(chip_button()),
                    widget::text(remove_label),
                    widget::tooltip::Position::Top,
                );
                row = row.push(chip);
            }

            scrollable(row)
                .direction(Direction::Horizontal(Scrollbar::hidden()))
                .id(TRAY_SCROLLABLE_ID.clone())
                .height(Length::Fixed(TRAY_SCROLLABLE_HEIGHT))
                .into()
        };

        // --- Row 2: count label | clear | copy button ---
        let count_label: Element<'_, Message> =
            widget::text(fl!("n-selected", count = count.to_string()))
                .size(FONT_SIZE_XS)
                .class(text_muted())
                .into();

        let clear_btn: Element<'_, Message> = if count > 0 {
            button::custom(widget::text(fl!("clear-all")).size(FONT_SIZE_XS))
                .on_press(Message::ClearSelection)
                .padding([SPACING_XS, SPACING_SM])
                .class(cosmic::theme::Button::Text)
                .into()
        } else {
            widget::Space::new().into()
        };

        let copy_label = if count > 0 {
            format!("{} ({})", fl!("copy"), count)
        } else {
            fl!("copy")
        };
        let copy_btn_content = widget::Row::new()
            .push(widget::icon(
                widget::icon::from_svg_bytes(ICON_COPY).symbolic(true),
            ))
            .push(widget::text(copy_label).size(FONT_SIZE_SM))
            .spacing(SPACING_XS)
            .align_y(Alignment::Center);

        let copy_btn = if count > 0 {
            button::custom(copy_btn_content)
                .on_press(Message::CopySelectedEmojis)
                .padding([SPACING_XS, SPACING_SM])
                .class(cosmic::theme::Button::Suggested)
        } else {
            button::custom(copy_btn_content)
                .padding([SPACING_XS, SPACING_SM])
                .class(cosmic::theme::Button::Standard)
        };

        let action_row = widget::Row::new()
            .push(count_label)
            .push(widget::space::horizontal())
            .push(clear_btn)
            .push(copy_btn)
            .spacing(SPACING_XS)
            .align_y(Alignment::Center);

        // --- Combine into tray ---
        let tray = container(
            widget::Column::new()
                .push(chips_row)
                .push(action_row)
                .spacing(SPACING_XS),
        )
        .padding([SPACING_SM, 0.0])
        .width(Length::Fill);

        Some(tray.into())
    }

    pub(crate) fn view_emoji_content(&self) -> Element<'_, Message> {
        let indices = &self.cached_indices;
        if indices.is_empty() {
            let (icon, title, subtitle) = if !emoji_data_loaded() {
                ("⚠️", fl!("emoji-data-error"), fl!("emoji-data-error-hint"))
            } else if !self.search_query.is_empty() {
                ("🔍", fl!("no-emojis-found"), fl!("try-different-search"))
            } else if self.selected_category == EmojiCategory::Recent {
                ("🕐", fl!("no-recent-emojis"), fl!("recent-hint"))
            } else {
                ("📭", fl!("no-emojis"), fl!("category-empty"))
            };
            return container(
                widget::Column::new()
                    .push(widget::text(icon).size(FONT_SIZE_EMPTY_ICON))
                    .push(widget::Space::new().height(SPACING_SM))
                    .push(widget::text(title).size(FONT_SIZE_MD))
                    .push(
                        widget::text(subtitle)
                            .size(FONT_SIZE_SM)
                            .class(text_muted()),
                    )
                    .spacing(SPACING_XS)
                    .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(cosmic::iced::alignment::Horizontal::Center)
            .align_y(cosmic::iced::alignment::Vertical::Center)
            .into();
        }

        // The same rows keyboard navigation moves through, so the two cannot disagree.
        let rows = self.layout_rows();
        let with_headers = self.shows_subcategories();
        let mut content = widget::Column::new().spacing(SPACING_SM);

        for group in rows.chunk_by(|a, b| a.group == b.group) {
            let subcategory = EMOJIS
                .get(indices[group[0].start])
                .and_then(|e| e.subcategory.as_deref());
            if let Some(subcat) = subcategory.filter(|_| with_headers) {
                content = content.push(
                    widget::text(subcat)
                        .size(FONT_SIZE_XS)
                        .line_height(cosmic::iced::Pixels(SUBCATEGORY_HEADER_HEIGHT))
                        .class(text_muted()),
                );
            }

            let grid =
                group
                    .iter()
                    .fold(widget::Column::new().spacing(GRID_SPACING), |grid, row| {
                        let buttons = indices[row.start..row.start + row.len]
                            .iter()
                            .filter_map(|&idx| self.emoji_button(idx));
                        grid.push(widget::Row::with_children(buttons).spacing(GRID_SPACING))
                    });
            content = content.push(grid);
        }

        scrollable(container(content).padding([SPACING_SM, SPACING_MD]))
            .id(EMOJI_SCROLLABLE_ID.clone())
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn emoji_button(&self, idx: usize) -> Option<Element<'_, Message>> {
        let Some(emoji) = EMOJIS.get(idx) else {
            log::warn!("Invalid emoji index {idx}, skipping");
            return None;
        };

        let display_emoji = self.get_display_emoji(idx);
        let is_selected = self.selected_indices.contains(&idx);
        let is_rejected = self.selection_rejected_idx == Some(idx);
        let is_highlighted = self.highlighted_index == Some(idx);

        let on_press = if self.multiselect_mode {
            Message::ToggleEmojiSelection(idx)
        } else {
            Message::EmojiSelected(idx)
        };

        let btn_class = if is_selected {
            cosmic::theme::Button::Suggested
        } else if is_rejected || is_highlighted {
            cosmic::theme::Button::Standard
        } else {
            cosmic::theme::Button::Text
        };

        let text_class = if is_rejected {
            text_muted()
        } else {
            cosmic::theme::Text::Default
        };

        let emoji_btn = button::custom(
            container(
                widget::text(display_emoji)
                    .size(FONT_SIZE_EMOJI)
                    .font(EMOJI_FONT)
                    .class(text_class),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(cosmic::iced::alignment::Horizontal::Center)
            .align_y(cosmic::iced::alignment::Vertical::Center),
        )
        .on_press(on_press)
        .width(Length::Fixed(EMOJI_BUTTON_SIZE))
        .height(Length::Fixed(EMOJI_BUTTON_SIZE))
        .name(emoji.name.as_str())
        .class(btn_class);

        Some(
            widget::tooltip(
                emoji_btn,
                widget::text(&emoji.name),
                widget::tooltip::Position::Top,
            )
            .into(),
        )
    }
}
