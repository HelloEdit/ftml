/*
 * render/html/word.rs
 *
 * ftml - Convert Wikidot code to HTML
 * Copyright (C) 2019 Ammon Smith for Project Foundation
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use percent_encoding::{percent_encode, DEFAULT_ENCODE_SET};
use self::Word::*;
use super::prelude::*;

macro_rules! percent_encode {
    ($input:expr) => ( percent_encode($input.as_ref(), DEFAULT_ENCODE_SET) )
}

pub fn render_words<'a, I, W>(ctx: &mut HtmlContext, words: I) -> Result<()>
where
    I: IntoIterator<Item = W>,
    W: AsRef<Word<'a>>,
{
    for word in words {
        render_word(ctx, word.as_ref())?;
    }

    Ok(())
}

// TODO remove this
#[allow(unused_variables)]
pub fn render_word(ctx: &mut HtmlContext, word: &Word) -> Result<()> {
    match word {
        &Anchor {
            href,
            name,
            id,
            class,
            style,
            target,
            ref words,
        } => {
            ctx.push_str("<a");

            if let Some(href) = href {
                write!(ctx.html, " href=\"{}\"", percent_encode!(href))?;
            }

            if let Some(name) = name {
                write!(ctx.html, " name=\"{}\"", name)?;
            }

            if let Some(id) = id {
                write!(ctx.html, " id=\"{}\"", id)?;
            }

            if let Some(class) = class {
                write!(ctx.html, " class=\"{}\"", class)?;
            }

            if let Some(style) = style {
                write!(ctx.html, " style=\"{}\"", style)?;
            }

            if let Some(target) = target {
                write!(ctx.html, " target=\"{}\"", target)?;
            }

            ctx.push('>');
            render_words(ctx, words)?;
            ctx.push_str("</a>");
        }
        &Link { href, target, text } => {
            write!(ctx.html, "<a href=\"{}\"", percent_encode!(href))?;

            if let Some(target) = target {
                write!(ctx.html, " target=\"{}\"", target)?;
            }

            // TODO fetch title of the page
            let text = match text {
                Some("") => "<page title here>",
                Some(text) => text,
                None => href,
            };

            ctx.push('>');
            escape_html(ctx, text)?;
            ctx.push_str("</a>");
        }
        &Bold { ref words } => {
            ctx.push_str("<b>");
            render_words(ctx, words)?;
            ctx.push_str("</b>");
        }
        &Color { color, ref words } => {
            ctx.push_str("<span style=\"color: ");
            escape_attr(ctx, color)?;
            ctx.push_str("\">");
            render_words(ctx, words)?;
            ctx.push_str("</span>");
        }
        &Date { timestamp, format } => {
            use chrono::prelude::*;

            // For now timestamp can only be seconds since the epoch.
            // In the future we may want to extend this to also include ISO timestamps and such.

            let date = NaiveDateTime::from_timestamp(timestamp, 0);
            let format = format.unwrap_or("%e");
            let (format, hover) = if format.ends_with("|agohover") {
                let new_len = format.len() - "|agohover".len();

                (&format[..new_len], true)
            } else {
                (format, false)
            };

            // TODO actually add the hover thing
            let _ = hover;

            write!(ctx.html, "{}", date.format(format))?;
        }
        &Email { address, text } => {
            write!(ctx.html, "<a href=\"mailto:{}\">", address)?;
            escape_html(ctx, text.unwrap_or(address))?;
            ctx.push_str("</a>");
        }
        &EquationReference { name } => unimplemented!(),
        &File { filename } => unimplemented!(),
        &Footnote { ref lines } => unimplemented!(),
        &FootnoteBlock => unimplemented!(),
        &Form { contents } => unimplemented!(),
        &Gallery => unimplemented!(),
        &Image {
            filename,
            float,
            direction,
            link,
            alt,
            title,
            width,
            height,
            style,
            class,
            size,
        } => {
            ctx.push_str("<div class=\"image-container\"");
            if let Some(align) = direction {
                write!(ctx.html, " style=\"text-align: {};\"", align)?;
            }
            ctx.push_str("><img");

            // TODO adjust for other sources
            write_tag_arg(ctx, "src", filename)?;

            // TODO float

            if let Some(alt) = alt {
                write!(ctx.html, " alt={}", alt)?;
            }

            // TODO title

            if let Some(width) = width {
                write!(ctx.html, " width={}", width)?;
            }

            if let Some(height) = height {
                write!(ctx.html, " height={}", height)?;
            }

            if let Some(style) = style {
                write!(ctx.html, " style={}", style)?;
            }

            if let Some(class) = class {
                write!(ctx.html, " class={}", class)?;
            }

            if let Some(size) = size {
                write!(ctx.html, " size={}", size)?;
            }

            ctx.push_str("></img></div>");
        }
        &Italics { ref words } => {
            ctx.push_str("<i>");
            render_words(ctx, words)?;
            ctx.push_str("</i>");
        }
        &Math { expr } => unimplemented!(),
        &Module {
            name,
            ref arguments,
            contents,
        } => unimplemented!(), // TODO switch to ctx vs ctx and add module listing
        &Monospace { ref words } => {
            ctx.push_str("<tt>");
            render_words(ctx, words)?;
            ctx.push_str("</tt>");
        }
        &Note { ref lines } => unimplemented!(),
        &Raw { contents } => escape_html(ctx, contents)?,
        &Size { size, ref lines } => {
            write!(ctx.html, "<span style=\"size: {};\">", size)?;
            render_lines(ctx, lines)?;
            ctx.push_str("</span>");
        }
        &Span {
            id,
            class,
            style,
            ref lines,
        } => {
            ctx.push_str("<span");

            if let Some(id) = id {
                write!(ctx.html, " id={}", id)?;
            }

            if let Some(class) = class {
                write!(ctx.html, " class={}", class)?;
            }

            if let Some(style) = style {
                write!(ctx.html, " style={}", style)?;
            }

            ctx.push('>');
            render_lines(ctx, lines)?;
            ctx.push_str("</span>");
        }
        &Strikethrough { ref words } => {
            ctx.push_str("<strike>");
            render_words(ctx, words)?;
            ctx.push_str("</strike>");
        }
        &Subscript { ref words } => {
            ctx.push_str("<sub>");
            render_words(ctx, words)?;
            ctx.push_str("</sub>");
        }
        &Superscript { ref words } => {
            ctx.push_str("<sup>");
            render_words(ctx, words)?;
            ctx.push_str("</sup>");
        }
        &TabList { ref tabs } => unimplemented!(),
        &Text { contents } => escape_html(ctx, contents)?,
        &Underline { ref words } => {
            ctx.push_str("<u>");
            render_words(ctx, words)?;
            ctx.push_str("</u>");
        }
        &User {
            username,
            show_picture,
        } => unimplemented!(),
    }

    Ok(())
}
