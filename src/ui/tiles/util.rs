use crate::{launcher::Launcher, CONFIG}; 
use gtk4::{ prelude::*, Box, Builder,TextView, Image, Label, ListBoxRow };
use std::collections::HashSet;

pub struct AsyncLauncherTile {
    pub launcher: Launcher,
    pub widget: ListBoxRow,
    pub title: Label,
    pub body: Label,
    pub attrs: Box,
}

pub fn ensure_icon_name(name: String) -> String {
    if name.ends_with("-symbolic") {
        name
    } else {
        format!("{}-symbolic", name)
    }
}


#[derive(Default)]
pub struct TextViewTileBuilder {
    pub object: Box,
    pub content: TextView,
}
impl TextViewTileBuilder {
    pub fn new(resource: &str)->Self{
        let builder = Builder::from_resource(resource);
        TextViewTileBuilder {
            object: builder.object("next_tile").unwrap_or_default(),
            content: builder.object("content").unwrap_or_default(),
        }
    }
}

#[derive(Default)]
pub struct TileBuilder {
    pub object: ListBoxRow,
    pub icon: Image,
    pub icon_holder: Box,
    pub title: Label,
    pub category: Label,
    pub attrs: Box,
    pub tag_start: Label,
    pub tag_end: Label,
    // Specific to 'bulk_text_tile'
    pub content_title: Label,
    pub content_body: Label,
    // Specific to 'calc_tile'
    pub equation_holder: Label,
    pub result_holder: Label,
}


impl TileBuilder {
    pub fn new(resource: &str, index: i32, show_shortcut: bool)->Self{
        let builder = Builder::from_resource(resource);
        let object: ListBoxRow = builder.object("holder").unwrap_or_default();
        let icon: Image = builder.object("icon-name").unwrap_or_default();
        let title: Label = builder.object("app-name").unwrap_or_default();
        let category: Label = builder.object("launcher-type").unwrap_or_default();
        let attrs: Box = builder.object("attrs-holder").unwrap_or_default();
        let icon_holder: Box = builder.object("app-icon-holder").unwrap_or_default();
        let tag_start: Label = builder.object("app-name-tag-start").unwrap_or_default();
        let tag_end: Label = builder.object("app-name-tag-end").unwrap_or_default();

        // Specific to 'bulk_text_tile' and 'error_tile'
        let content_title: Label = builder.object("content-title").unwrap_or_default();
        let content_body: Label = builder.object("content-body").unwrap_or_default();

        // Specific to 'calc_tile'
        let equation_holder: Label = builder.object("equation-holder").unwrap_or_default();
        let result_holder: Label = builder.object("result-holder").unwrap_or_default();
    
        // Implement the shortcuts
        if show_shortcut && index < 5 {
            let shortcut_holder: Box = builder.object("shortcut-holder").unwrap_or_default();
            let shortcut: Label = builder.object("shortcut").unwrap_or_default();
            shortcut_holder.set_visible(true);
            shortcut.set_text(format!("ctrl + {}", index + 1).as_str());
        }

        // Set the icon size to the user-specified one
        if let Some(c) = CONFIG.get(){
            icon.set_pixel_size(c.appearance.icon_size);
        }

        TileBuilder {
            object,
            icon,
            icon_holder,
            title,
            category,
            attrs,
            tag_start,
            tag_end,

            content_body,
            content_title,

            equation_holder,
            result_holder,
        }

    }
    pub fn add_default_attrs<T: AsRef<str>>(&self, method: Option<T>, result:Option<T>, keyword: Option<T>, exec: Option<T>, additional_attrs: Option<Vec<(&str, &str)>>){
        let method = method.as_ref().map(|s| ("method", s.as_ref()));
        let result = result.as_ref().map(|s| ("result", s.as_ref()));
        let exec = exec.as_ref().map(|s| ("exec", s.as_ref()));
        let keyword = keyword.as_ref().map(|s| ("keyword", s.as_ref()));

        let mut attrs: Vec<(&str, &str)> = vec![method, result, exec, keyword].into_iter().filter_map(|x| x).collect();
        if let Some(ads) = additional_attrs {
            attrs.extend(ads);
        }
        for item in attrs {
            let (key, value, ..) = item;
            let label = Label::new(Some(format!("{} | {}", key, value).as_str()));
            self.attrs.append(&label);
        }
        
    }
    pub fn display_tag_start<T>(&self, content: &Option<String>, keyword: T)
        where T: AsRef<str>
    {
        if let Some(start_tag) = content {
            let text = start_tag.replace("{keyword}", keyword.as_ref());
            if !text.is_empty(){
                self.tag_start.set_text(&text);
                self.tag_start.set_visible(true);
            }
        }
    }
    pub fn display_tag_end<T>(&self, content: &Option<String>, keyword: T)
        where T: AsRef<str>
    {
        if let Some(start_tag) = content {
            let text = start_tag.replace("{keyword}", keyword.as_ref());
            if !text.is_empty(){
                self.tag_end.set_text(&text);
                self.tag_end.set_visible(true);
            }
        }
    }
}



pub trait SherlockSearch {
    fn fuzzy_match<T>(&self, substring: T) -> bool
    where
        Self: AsRef<str>,
        T: AsRef<str>,
    {
        let char_pattern: HashSet<char> = substring.as_ref().chars().collect();
        let concat_str: String = self
            .as_ref()
            .chars()
            .filter(|s| char_pattern.contains(s))
            .collect();
        concat_str.contains(substring.as_ref())
    }
}
impl SherlockSearch for String {}
