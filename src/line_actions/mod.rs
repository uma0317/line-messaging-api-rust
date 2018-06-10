use structs::Rectangle;

pub enum LineActionType {
    Uri,
    Message,
    Postback
}

pub enum ImageActionType {
    ImagemapURIAction { link_url: String },
    ImagemapMessageAction {text: String},

}
pub struct ImagemapAction {
    kind: ImageActionType,
    area: Rectangle,
}

impl ImagemapAction {
    pub fn new(kind: ImageActionType, area: Rectangle) -> ImagemapAction {
        ImagemapAction { kind: kind, area: area}
    }
}

pub enum TemplateActionType {
    TemplateURIAction { uri: String },
    TemplateMessageAction { text: String },
    TemplatePostbackAction { text: String, data: String }
}

pub struct TemplateAction {
    kind:  TemplateActionType,
    label: String,
}

impl TemplateAction {
    pub fn new(kind: TemplateActionType, label: String) -> TemplateAction {
        TemplateAction { kind, label }
    }
}