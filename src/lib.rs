// lib.rs
// As part of the spark project
// Created by Maxims Enterprise in 2024

pub mod lexer {
    pub mod tokens;
    pub mod lexer;
}

pub mod parser {
    pub mod nodes;
    pub mod parser;
    pub mod operations;
}

pub mod program;

pub mod interpreter {
    pub mod interpreter;
    pub mod interpreter_utils;
    pub mod components {
        pub mod canvas;
        pub mod title;
        pub mod paragraph;
        pub mod script;
        pub mod abbreviation;
        pub mod div;
        pub mod text;
        pub mod link;
        pub mod address;
        pub mod article;
        pub mod aside;
        pub mod footer;
        pub mod header;
        pub mod content;
        pub mod nav;
        pub mod section;
        pub mod search;
        pub mod quote;
        pub mod description {
            pub mod dd;
            pub mod dl;
            pub mod dt;
        }
        pub mod media {
            pub mod figure_caption;
            pub mod figure;
            pub mod area;
            pub mod audio;
            pub mod image;
            pub mod map;
            pub mod track;
            pub mod video;
        }
        pub mod embed {
            pub mod embed;
            pub mod iframe;
            pub mod object;
            pub mod picture;
            pub mod source;
        }
        pub mod inline {
            pub mod br;
            pub mod hr;
            pub mod emphasis;
            pub mod i;
            pub mod keyboard_input;
            pub mod mark;
            pub mod q;
            pub mod s;
            pub mod samp;
            pub mod small;
            pub mod span;
            pub mod strong;
            pub mod sub;
            pub mod sup;
            pub mod time;
            pub mod u;
            pub mod var;
            pub mod wbr;
            pub mod ins;
            pub mod del;
        }
        pub mod groups {
            pub mod li;
            pub mod menu;
            pub mod ol;
            pub mod cite;
        }
        pub mod ruby {
            pub mod ruby;
            pub mod rp;
            pub mod rt;
        }
        pub mod code;
        pub mod data;
        pub mod pre;
        pub mod b;
        pub mod definition;
        pub mod bdi;
        pub mod bdo;
    }
}

pub mod formatter {
    pub mod formatter;
}

pub mod utils {
    pub mod error;
}

pub mod compile;