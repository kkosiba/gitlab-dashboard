use std::cmp::{max, min};

pub fn build_paginator(total_pages: usize, current_page: usize) -> String {
    let mut pagination = String::new();

    // Add the "previous page" marker
    pagination.push_str("<H ");

    // Add the first page
    if current_page == 1 {
        pagination.push_str("[1] ");
    } else {
        pagination.push_str("1 ");
    }

    // Add ellipsis if needed
    if current_page > 3 {
        pagination.push_str("... ");
    }

    // Add the current page or nearby pages
    for page in max(2, current_page - 1)..=min(total_pages - 1, current_page + 1) {
        if page == current_page {
            pagination.push_str(&format!("[{}] ", page));
        } else {
            pagination.push_str(&format!("{} ", page));
        }
    }

    // Add ellipsis if needed
    if current_page < total_pages - 2 {
        pagination.push_str("... ");
    }

    // Add the last page
    if total_pages > 1 {
        pagination.push_str(&format!("{} ", total_pages));
    }

    // Add the "next page" marker
    pagination.push_str("L>");

    pagination
}
