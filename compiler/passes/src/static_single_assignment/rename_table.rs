// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use leo_span::Symbol;

use indexmap::IndexMap;

/// `RenameTable` tracks the names assigned by static single assignment in a single block.
// Developer Note: `RenameTable` is kept distinct from `SymbolTable` for the purposes of this prototype.
// However, its functionality could be folded into `SymbolTable` after considering the implications of the design.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct RenameTable {
    /// The `RenameTable` of the parent block.
    pub(crate) parent: Option<Box<RenameTable>>,
    /// The mapping from names in the original AST to new names in the renamed AST.
    pub(crate) mapping: IndexMap<Symbol, Symbol>,
}

impl RenameTable {
    /// Returns the symbols that were renamed in the current basic block.
    pub fn get_local_names(&self) -> Vec<&Symbol> {
        self.mapping.keys().collect()
    }

    /// Updates `self.mapping` with the desired entry. Creates a new entry if `symbol` is not already in `self.mapping`.
    pub fn update(&mut self, symbol: Symbol, new_symbol: Symbol) {
        self.mapping.insert(symbol, new_symbol);
    }

    /// Looks up the new name for `symbol`, recursively checking the parent if it is not found.
    pub fn lookup(&self, symbol: &Symbol) -> Option<&Symbol> {
        if let Some(var) = self.mapping.get(symbol) {
            Some(var)
        } else if let Some(parent) = &self.parent {
            parent.lookup(symbol)
        } else {
            None
        }
    }
}
