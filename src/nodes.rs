use std::str::FromStr;

use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::parser::Rule;

#[derive(Debug, Clone)]
pub struct ParseRoot {
    children: Vec<ParseNode>,
}

impl ParseRoot {
    pub fn new(children: Vec<ParseNode>) -> Self {
        Self { children }
    }

    pub fn children(&self) -> &Vec<ParseNode> {
        &self.children
    }

    pub fn children_owned(self) -> Vec<ParseNode> {
        self.children
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNode {
    kind: MdParseEnum,
    content: String,
    children: Vec<ParseNode>,
}

impl ParseNode {
    pub fn new(kind: MdParseEnum, content: String) -> Self {
        Self {
            kind,
            content,
            children: Vec::new(),
        }
    }

    pub fn kind(&self) -> MdParseEnum {
        self.kind
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn add_children(&mut self, children: Vec<ParseNode>) {
        self.children.extend(children);
    }

    pub fn children(&self) -> &Vec<ParseNode> {
        &self.children
    }

    pub fn children_owned(self) -> Vec<ParseNode> {
        self.children
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MdParseEnum {
    Heading,
    Word,
    Task,
    TaskOpen,
    TaskClosed,
    UnorderedList,
    ListContainer,
    OrderedList,
    CodeBlock,
    PLanguage,
    CodeStr,
    Code,
    Paragraph,
    Link,
    LinkData,
    Quote,
    Table,
    TableSeperator,
    TableRow,
    Digit,
    BlockSeperator,
    Sentence,
    Bold,
    Italic,
    Strikethrough,
    HorizontalSeperator,
}

impl From<Rule> for MdParseEnum {
    fn from(value: Rule) -> Self {
        match value {
            Rule::word | Rule::table_word => Self::Word,
            Rule::italic | Rule::italic_word => Self::Italic,
            Rule::bold | Rule::bold_word => Self::Bold,
            Rule::strikethrough | Rule::strikethrough_word => Self::Strikethrough,
            Rule::code_word | Rule::code => Self::Code,
            Rule::programming_language => Self::PLanguage,
            Rule::link_word | Rule::markdown_link | Rule::external_link | Rule::link => Self::Link,
            Rule::o_list_counter => Self::Digit,
            Rule::task_open => Self::TaskOpen,
            Rule::task_complete => Self::TaskClosed,
            Rule::code_line | Rule::sentence => Self::Sentence,
            Rule::table_row => Self::TableRow,
            Rule::table_seperator => Self::TableSeperator,
            Rule::u_list => Self::UnorderedList,
            Rule::o_list => Self::OrderedList,
            Rule::h1 | Rule::h2 | Rule::h3 | Rule::h4 | Rule::heading => Self::Heading,
            Rule::list_container => Self::ListContainer,
            Rule::paragraph => Self::Paragraph,
            Rule::code_block => Self::CodeBlock,
            Rule::table => Self::Table,
            Rule::quote => Self::Quote,
            Rule::task => Self::Task,
            Rule::block_sep => Self::BlockSeperator,
            Rule::horizontal_sep => Self::HorizontalSeperator,
            Rule::link_data => Self::LinkData,
            Rule::norwegian_char
            | Rule::char
            | Rule::table_char
            | Rule::link_char
            | Rule::digit
            | Rule::normal
            | Rule::comment
            | Rule::txt
            | Rule::task_prefix
            | Rule::quote_prefix
            | Rule::code_block_prefix
            | Rule::table_prefix
            | Rule::list_prefix
            | Rule::forbidden_sentence_prefic => Self::Paragraph,
        }
    }
}

impl FromStr for MdParseEnum {
    type Err = ();

    /// This cannot return Err
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "h1" | "h2" | "h3" | "h4" | "heading" => Ok(Self::Heading),
            "task" => Ok(Self::Task),
            "task_open" => Ok(Self::TaskOpen),
            "task_complete" => Ok(Self::TaskClosed),
            "u_list" => Ok(Self::UnorderedList),
            "o_list" => Ok(Self::OrderedList),
            "code_block" => Ok(Self::CodeBlock),
            "programming_language" => Ok(Self::PLanguage),
            "code" | "code_word" => Ok(Self::Code),
            "paragraph" => Ok(Self::Paragraph),
            "link" => Ok(Self::Link),
            "quote" => Ok(Self::Quote),
            "table" => Ok(Self::Table),
            "table_seperator" => Ok(Self::TableSeperator),
            "table_row" => Ok(Self::TableRow),
            "block_sep" => Ok(Self::BlockSeperator),
            "code_line" => Ok(Self::Sentence),
            "list_container" => Ok(Self::ListContainer),
            "table_word" | "o_list_counter" | "word" | "digit" => Ok(Self::Word),
            "bold" | "bold_word" => Ok(Self::Bold),
            "italic" | "italic_word" => Ok(Self::Italic),
            "strikethrough" | "strikethrough_word" => Ok(Self::Strikethrough),
            _e => {
                Ok(Self::Paragraph)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RenderRoot {
    components: Vec<RenderComponent>,
}

impl RenderRoot {
    pub fn new(components: Vec<RenderComponent>) -> Self {
        Self { components }
    }

    pub fn components(&self) -> &Vec<RenderComponent> {
        &self.components
    }

    pub fn components_owned(self) -> Vec<RenderComponent> {
        self.components
    }

    pub fn components_mut(&mut self) -> &mut Vec<RenderComponent> {
        &mut self.components
    }

    /// Sets the y offset of the components
    pub fn set_scroll(&mut self, scroll: u16) {
        let mut y_offset = 0;
        for component in self.components.iter_mut() {
            component.set_y_offset(y_offset);
            component.set_scroll_offset(scroll);
            y_offset += component.height();
        }
    }

    /// Transforms the content of the components to fit the given width
    pub fn transform(&mut self, width: u16) {
        for component in self.components_mut() {
            component.transform(width);
        }
    }

    pub fn height(&self) -> u16 {
        self.components.iter().map(|c| c.height()).sum()
    }
}

impl Widget for RenderRoot {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for component in self.components_owned() {
            component.render(area, buf);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordType {
    MetaInfo,
    Normal,
    Code,
    Link,
    Italic,
    Bold,
    Strikethrough,
}

impl From<MdParseEnum> for WordType {
    fn from(value: MdParseEnum) -> Self {
        match value {
            MdParseEnum::PLanguage
            | MdParseEnum::BlockSeperator
            | MdParseEnum::TaskOpen
            | MdParseEnum::TaskClosed
            | MdParseEnum::LinkData
            | MdParseEnum::HorizontalSeperator => WordType::MetaInfo,

            MdParseEnum::Code => WordType::Code,
            MdParseEnum::Bold => WordType::Bold,
            MdParseEnum::Italic => WordType::Italic,
            MdParseEnum::Strikethrough => WordType::Strikethrough,
            MdParseEnum::Link => WordType::Link,

            MdParseEnum::Paragraph
            | MdParseEnum::TableRow
            | MdParseEnum::Digit
            | MdParseEnum::Sentence
            | MdParseEnum::Word => WordType::Normal,

            MdParseEnum::Heading
            | MdParseEnum::Task
            | MdParseEnum::UnorderedList
            | MdParseEnum::ListContainer
            | MdParseEnum::OrderedList
            | MdParseEnum::CodeBlock
            | MdParseEnum::CodeStr
            | MdParseEnum::Quote
            | MdParseEnum::Table
            | MdParseEnum::TableSeperator => unreachable!("{:?}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Word {
    content: String,
    word_type: WordType,
}

impl Word {
    pub fn new(content: String, word_type: WordType) -> Self {
        Self { content, word_type }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut String {
        &mut self.content
    }

    pub fn set_content(&mut self, content: impl Into<String>) {
        self.content = content.into();
    }

    pub fn kind(&self) -> WordType {
        self.word_type
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RenderNode {
    Paragraph,
    LineBreak,
    Heading,
    Task,
    List,
    Table,
    CodeBlock,
    Quote,
}

#[derive(Debug, Clone)]
pub struct RenderComponent {
    kind: RenderNode,
    content: Vec<Vec<Word>>,
    meta_info: Vec<Word>,
    height: u16,
    offset: u16,
    scroll_offset: u16,
}

impl RenderComponent {
    pub fn new(kind: RenderNode, content: Vec<Word>) -> Self {
        let meta_info: Vec<Word> = content
            .iter()
            .filter(|c| c.kind() == WordType::MetaInfo)
            .cloned()
            .collect();

        Self {
            kind,
            content: vec![content],
            meta_info,
            height: 0,
            offset: 0,
            scroll_offset: 0,
        }
    }

    pub fn new_formatted(kind: RenderNode, content: Vec<Vec<Word>>) -> Self {
        let meta_info: Vec<Word> = content
            .iter()
            .flatten()
            .filter(|c| c.kind() == WordType::MetaInfo)
            .cloned()
            .collect();

        Self {
            kind,
            height: content.len() as u16,
            meta_info,
            content,
            offset: 0,
            scroll_offset: 0,
        }
    }

    pub fn kind(&self) -> RenderNode {
        self.kind
    }

    pub fn content(&self) -> &Vec<Vec<Word>> {
        &self.content
    }

    pub fn content_mut(&mut self) -> &mut Vec<Vec<Word>> {
        &mut self.content
    }

    pub fn meta_info(&self) -> &Vec<Word> {
        &self.meta_info
    }

    pub fn content_owned(self) -> Vec<Vec<Word>> {
        self.content
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn y_offset(&self) -> u16 {
        self.offset
    }

    pub fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }

    pub fn set_y_offset(&mut self, y_offset: u16) {
        self.offset = y_offset;
    }

    pub fn set_scroll_offset(&mut self, offset: u16) {
        self.scroll_offset = offset;
    }

    pub fn transform(&mut self, width: u16) {
        match self.kind {
            RenderNode::Heading => self.height = 1,
            RenderNode::List => {
                self.content.iter().len();
            }
            RenderNode::CodeBlock => {
                let height = self
                    .content
                    .iter()
                    .filter(|c| c.iter().any(|x| x.kind() != WordType::MetaInfo))
                    .count() as u16;
                self.height = height;
            }
            RenderNode::Paragraph | RenderNode::Task => {
                let width = match self.kind {
                    RenderNode::Paragraph => width as usize,
                    RenderNode::Task => width as usize - 4,
                    _ => unreachable!(),
                };
                let mut len = 0;
                let mut lines = Vec::new();
                let mut line = Vec::new();
                let mut last_kind = WordType::Normal;
                for word in self.content.iter().flatten() {
                    if word.content.len() + len < width && !line.is_empty() {
                        if word.kind() == WordType::Normal
                            && !word.content.starts_with(' ')
                            && last_kind != WordType::Normal
                        {
                            line.push(Word::new(" ".to_owned(), WordType::Normal));
                        }
                        line.push(word.clone());
                        len += word.content.len() + 1;
                    } else {
                        if line.is_empty() {
                            line.push(word.clone());
                            len += word.content.len() + 1;
                            continue;
                        }
                        lines.push(line);
                        len = word.content.len() + 1;
                        let mut word = word.clone();
                        let content = word.content.trim_start().to_owned();
                        word.set_content(content);
                        line = vec![word];
                    }
                    last_kind = word.kind();
                }
                if !line.is_empty() {
                    lines.push(line);
                }
                self.height = lines.len() as u16;
                self.content = lines;
            }
            RenderNode::LineBreak => {
                self.height = 1;
            }
            RenderNode::Table => {
                let height = self.content.len() as u16;
                self.height = height;
            }
            RenderNode::Quote => todo!(),
        }
    }
}
