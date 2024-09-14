use makepad_widgets::Cursor;

/// The kind of edit
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditKind {
    /// A character was inserted.
    Insert,
    /// A character was deleted. figure up the backspace
    Backspace,
    /// A character was deleted. figure up the backspace and delete keys
    Delete,
    /// other edit kind
    Other,
}

impl EditKind {
    pub fn can_merge_with(self, other: EditKind) -> bool {
        if self == Self::Other {
            false
        } else {
            self == other
        }
    }
}

// ------------------------------------------------------------------------------------------------------------

/// An edit that was made to the text.
#[derive(Clone, Debug)]
pub struct Edit {
    /// the start of the edit selection
    pub start: usize,
    /// the end of the edit selection
    pub end: usize,
    /// replace str
    pub replace_with: String,
}

impl Edit {
    pub fn apply(&self, text: &mut String) {
        text.replace_range(self.start..self.end, &self.replace_with);
    }

    pub fn invert(&self, text: &str) -> Self {
        Self {
            start: self.start,
            end: self.start + self.replace_with.len(),
            replace_with: text[self.start..self.end].to_string(),
        }
    }
}

// ------------------------------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct EditGroup {
    cursor: Cursor,
    edit_start: usize,
}

// -------------------------------------------------------------------------------------------------------------

/// A stack of edits
#[derive(Clone, Debug, Default)]
pub struct EditStack {
    edit_groups: Vec<EditGroup>,
    edits: Vec<Edit>,
}

impl EditStack {
    fn push_edit_group(&mut self, cursor: Cursor) {
        self.edit_groups.push(EditGroup {
            cursor,
            edit_start: self.edits.len(),
        });
    }

    fn push_edit(&mut self, edit: Edit) {
        self.edits.push(edit);
    }

    fn pop_edit_group(&mut self, edits: &mut Vec<Edit>) -> Option<Cursor> {
        match self.edit_groups.pop() {
            Some(edit_group) => {
                edits.extend(self.edits.drain(edit_group.edit_start..).rev());
                Some(edit_group.cursor)
            }
            None => None,
        }
    }

    fn clear(&mut self) {
        self.edit_groups.clear();
        self.edits.clear();
    }
}

// -------------------------------------------------------------------------------------------------------------
#[derive(Clone, Debug, Default)]
pub struct History {
    current_edit_kind: Option<EditKind>,
    undo_stack: EditStack,
    redo_stack: EditStack,
}

impl History {
    pub fn last_inserted_text<'a>(&self, text: &'a str) -> Option<&'a str> {
        self.undo_stack
            .edits
            .last()
            .map(|edit| &text[edit.start..edit.end])
    }

    pub fn force_new_edit_group(&mut self) {
        self.current_edit_kind = None;
    }

    pub fn create_or_extend_edit_group(&mut self, edit_kind: EditKind, cursor: Cursor) {
        if !self.current_edit_kind.map_or(false, |current_edit_kind| {
            current_edit_kind.can_merge_with(edit_kind)
        }) {
            self.undo_stack.push_edit_group(cursor);
            self.current_edit_kind = Some(edit_kind);
        }
    }

    pub fn apply_edit(&mut self, edit: Edit, text: &mut String) {
        let inverted_edit = edit.invert(&text);
        edit.apply(text);
        // dbg!(&edit, &text, &inverted_edit);
        self.undo_stack.push_edit(inverted_edit);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, cursor: Cursor, text: &mut String) -> Option<Cursor> {
        let mut edits = Vec::new();
        if let Some(new_cursor) = self.undo_stack.pop_edit_group(&mut edits) {
            self.redo_stack.push_edit_group(cursor);
            for edit in edits {
                let inverted_edit = edit.clone().invert(text);
                edit.apply(text);
                self.redo_stack.push_edit(inverted_edit);
            }
            self.current_edit_kind = None;
            Some(new_cursor)
        } else {
            None
        }
    }

    pub fn redo(&mut self, cursor: Cursor, text: &mut String) -> Option<Cursor> {
        let mut edits = Vec::new();
        if let Some(new_cursor) = self.redo_stack.pop_edit_group(&mut edits) {
            self.undo_stack.push_edit_group(cursor);
            for edit in edits {
                let inverted_edit = edit.clone().invert(text);
                edit.apply(text);
                self.undo_stack.push_edit(inverted_edit);
            }
            self.current_edit_kind = None;
            Some(new_cursor)
        } else {
            None
        }
    }
    pub fn clear(&mut self) {
        self.current_edit_kind = None;
        self.undo_stack.clear();
        self.redo_stack.clear();
    }
}
