// svgcleaner could help you to clean up your SVG files
// from unnecessary data.
// Copyright (C) 2012-2017 Evgeniy Reizner
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License along
// with this program; if not, write to the Free Software Foundation, Inc.,
// 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

use svgdom::Document;
use svgdom::postproc;

use task::short::EId;

pub fn fix_invalid_attributes(doc: &Document) {
    for mut node in doc.descendants().svg() {
        // We are iterating only over svg elements, which all have a tag name.
        match node.tag_id().unwrap() {
            EId::Rect => postproc::fix_rect_attributes(&mut node),
            EId::Polyline | EId::Polygon => postproc::fix_poly_attributes(&mut node),
            EId::LinearGradient | EId::RadialGradient => postproc::fix_stop_attributes(&mut node),
            _ => {}
        }
    }
}
