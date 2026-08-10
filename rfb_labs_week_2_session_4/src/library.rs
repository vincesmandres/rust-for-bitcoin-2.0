use crate::catalogue::{Item, LoanStatus, LoanTerms};
use crate::error::LibraryError;
use crate::member::Member;

pub const MAX_ITEMS_PER_MEMBER: usize = 3;

/// Owns every item and every member.
///
/// The fields are private because the library is responsible for keeping an
/// item's `LoanStatus` and a member's borrowed-id list in agreement. Callers
/// reach the data through the borrowing lookups below.
// TODO(Part 3): delete this attribute once your lookups actually read the
// fields. It is here only so the untouched starter crate compiles clean.

#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Library {
    items: Vec<Item>,
    members: Vec<Member>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), LibraryError> {
        // TODO(Part 3): move `item` into the library. Reject an empty title
        // and an id that is already stocked.
        if item.title.trim().is_empty() {
            return Err(LibraryError::EmptyTitle);
        }
        
        if self.items.iter().any(|existing| existing.id == item.id) {
            return Err(LibraryError::DuplicateItemId { id: item.id });
        }

        self.items.push(item);
        Ok(())
    }

    pub fn register_member(&mut self, member: Member) -> Result<(), LibraryError> {
        // TODO(Part 3): move `member` in. Reject an id already registered.
        if self.members.iter().any(|existing| existing.id == member.id) {
            return Err(
                LibraryError::DuplicateMemberId { 
                    id: member.id });
        }
        self.members.push(member);
        Ok(())
    }

    pub fn find_item(&self, id: u32) -> Option<&Item> {
        // TODO(Part 3): borrow from `self`; do not clone.
        self.items.iter().find(|item| item.id == id)
    }

    pub fn find_member(&self, id: u32) -> Option<&Member> {
        // TODO(Part 3)
        self.members.iter().find(|member| member.id == id)
    }

    pub fn items_by_author<'a>(&'a self, author: &str) -> Vec<&'a Item> {
        // TODO(Part 3): return references to all matching items.
        // This is a borrowing + lifetimes problem ajajajaja 
        self.items
            .iter()
            .filter(|item| item.author == author)
            .collect()
    }

    pub fn available_items(&self) -> Vec<&Item> {
        // TODO(Part 3)
        self.items
            .iter()
            .filter(|item| item.status == LoanStatus::Available)
            .collect()
    }

    pub fn longest_loan_item(&self) -> Option<&Item> {
        // TODO(Part 4): the item that may be kept longest, via `LoanTerms`.
        self.items.iter().max_by_key(|item| item.loan_days())
    }

    pub fn checkout(
    &mut self,
    item_id: u32,
    member_id: u32,
    day: u32,
) -> Result<(), LibraryError> {
    // 1. Find item index
    let item_index = self
        .items
        .iter()
        .position(|item| item.id == item_id)
        .ok_or(LibraryError::ItemNotFound { id: item_id })?;

    // 2. Find member index
    let member_index = self
        .members
        .iter()
        .position(|member| member.id == member_id)
        .ok_or(LibraryError::MemberNotFound { id: member_id })?;

    // 3. Validate item status
    match self.items[item_index].status {
        LoanStatus::Lost => {
            return Err(LibraryError::ItemIsLost { id: item_id });
        }

        LoanStatus::OnLoan {
            member_id: current_member_id,
            ..
        } => {
            return Err(LibraryError::ItemAlreadyOnLoan {
                id: item_id,
                member_id: current_member_id,
            });
        }

        LoanStatus::Available => {}
    }

    // 4. Validate borrowing limit
    if self.members[member_index].borrowed_item_ids.len() >= MAX_ITEMS_PER_MEMBER {
        return Err(LibraryError::BorrowLimitReached {
            member_id,
            limit: MAX_ITEMS_PER_MEMBER,
        });
    }

    // 5. Update item
    self.items[item_index].status = LoanStatus::OnLoan {
        member_id,
        day_borrowed: day,
    };

    // 6. Update member
    self.members[member_index]
        .borrowed_item_ids
        .push(item_id);

    Ok(())
}

    /// Returns the late fee owed, in cents.
    pub fn return_item(&mut self, item_id: u32, day: u32) -> Result<u32, LibraryError> {
        // TODO(Part 6): checked subtraction must return InvalidReturnDay.
        let _ = (item_id, day);
        todo!("return an item")
    }
}
