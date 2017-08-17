#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
extern crate pest;
extern crate regex;

use pest::prelude::*;
use std::collections::{HashMap, VecDeque};

pub enum Pat {
    Rule(String),
    Token(GrammarToken),
    Regex(String),
    Seq(Vec<Pat>),
    Cap(Capture, Box<Pat>),
    Opt(Box<Pat>),
    ZeroPlus(Box<Pat>),
    OnePlus(Box<Pat>),
    AnyOf(Vec<Pat>),
    Loop(Box<Pat>),
    BreakOnToken(GrammarToken),
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Pat {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Pat::Rule(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Rule");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::Token(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Token");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::Regex(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Regex");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::Seq(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Seq");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::Cap(ref __self_0, ref __self_1),) => {
                let mut builder = __arg_0.debug_tuple("Cap");
                let _ = builder.field(&&(*__self_0));
                let _ = builder.field(&&(*__self_1));
                builder.finish()
            }
            (&Pat::Opt(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Opt");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::ZeroPlus(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("ZeroPlus");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::OnePlus(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("OnePlus");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::AnyOf(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("AnyOf");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::Loop(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Loop");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Pat::BreakOnToken(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("BreakOnToken");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Pat {
    #[inline]
    fn clone(&self) -> Pat {
        match (&*self,) {
            (&Pat::Rule(ref __self_0),) =>
            Pat::Rule(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::Token(ref __self_0),) =>
            Pat::Token(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::Regex(ref __self_0),) =>
            Pat::Regex(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::Seq(ref __self_0),) =>
            Pat::Seq(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::Cap(ref __self_0, ref __self_1),) =>
            Pat::Cap(::std::clone::Clone::clone(&(*__self_0)),
                     ::std::clone::Clone::clone(&(*__self_1))),
            (&Pat::Opt(ref __self_0),) =>
            Pat::Opt(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::ZeroPlus(ref __self_0),) =>
            Pat::ZeroPlus(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::OnePlus(ref __self_0),) =>
            Pat::OnePlus(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::AnyOf(ref __self_0),) =>
            Pat::AnyOf(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::Loop(ref __self_0),) =>
            Pat::Loop(::std::clone::Clone::clone(&(*__self_0))),
            (&Pat::BreakOnToken(ref __self_0),) =>
            Pat::BreakOnToken(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}

pub struct GrammarRule {
    pub name: String,
    pub pat: Pat,
    pub nof_captures: usize,
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for GrammarRule {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            GrammarRule {
            name: ref __self_0_0,
            pat: ref __self_0_1,
            nof_captures: ref __self_0_2 } => {
                let mut builder = __arg_0.debug_struct("GrammarRule");
                let _ = builder.field("name", &&(*__self_0_0));
                let _ = builder.field("pat", &&(*__self_0_1));
                let _ = builder.field("nof_captures", &&(*__self_0_2));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for GrammarRule {
    #[inline]
    fn clone(&self) -> GrammarRule {
        match *self {
            GrammarRule {
            name: ref __self_0_0,
            pat: ref __self_0_1,
            nof_captures: ref __self_0_2 } =>
            GrammarRule{name: ::std::clone::Clone::clone(&(*__self_0_0)),
                        pat: ::std::clone::Clone::clone(&(*__self_0_1)),
                        nof_captures:
                            ::std::clone::Clone::clone(&(*__self_0_2)),},
        }
    }
}

pub enum GrammarToken { Str(String), Re(String), }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for GrammarToken {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&GrammarToken::Str(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Str");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&GrammarToken::Re(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Re");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for GrammarToken {
    #[inline]
    fn clone(&self) -> GrammarToken {
        match (&*self,) {
            (&GrammarToken::Str(ref __self_0),) =>
            GrammarToken::Str(::std::clone::Clone::clone(&(*__self_0))),
            (&GrammarToken::Re(ref __self_0),) =>
            GrammarToken::Re(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}

pub enum Capture { Unnamed, Shared(usize), Named(usize), }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Capture {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Capture::Unnamed,) => {
                let mut builder = __arg_0.debug_tuple("Unnamed");
                builder.finish()
            }
            (&Capture::Shared(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Shared");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
            (&Capture::Named(ref __self_0),) => {
                let mut builder = __arg_0.debug_tuple("Named");
                let _ = builder.field(&&(*__self_0));
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Capture {
    #[inline]
    fn clone(&self) -> Capture {
        match (&*self,) {
            (&Capture::Unnamed,) => Capture::Unnamed,
            (&Capture::Shared(ref __self_0),) =>
            Capture::Shared(::std::clone::Clone::clone(&(*__self_0))),
            (&Capture::Named(ref __self_0),) =>
            Capture::Named(::std::clone::Clone::clone(&(*__self_0))),
        }
    }
}

pub enum Quantifier { Opt, ZeroPlus, OnePlus, Loop, BreakOnToken, }
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::fmt::Debug for Quantifier {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Quantifier::Opt,) => {
                let mut builder = __arg_0.debug_tuple("Opt");
                builder.finish()
            }
            (&Quantifier::ZeroPlus,) => {
                let mut builder = __arg_0.debug_tuple("ZeroPlus");
                builder.finish()
            }
            (&Quantifier::OnePlus,) => {
                let mut builder = __arg_0.debug_tuple("OnePlus");
                builder.finish()
            }
            (&Quantifier::Loop,) => {
                let mut builder = __arg_0.debug_tuple("Loop");
                builder.finish()
            }
            (&Quantifier::BreakOnToken,) => {
                let mut builder = __arg_0.debug_tuple("BreakOnToken");
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::std::clone::Clone for Quantifier {
    #[inline]
    fn clone(&self) -> Quantifier {
        match (&*self,) {
            (&Quantifier::Opt,) => Quantifier::Opt,
            (&Quantifier::ZeroPlus,) => Quantifier::ZeroPlus,
            (&Quantifier::OnePlus,) => Quantifier::OnePlus,
            (&Quantifier::Loop,) => Quantifier::Loop,
            (&Quantifier::BreakOnToken,) => Quantifier::BreakOnToken,
        }
    }
}
















// unpack first, since it's optional






pub struct Rdp<T> {
    input: T,
    queue: Vec<Token<Rule>>,
    queue_index: ::std::cell::Cell<usize>,
    failures: Vec<Rule>,
    fail_pos: usize,
    stack: Vec<String>,
    atomic: bool,
    eoi_matched: bool,
}
#[allow(dead_code, non_camel_case_types)]
#[structural_match]
#[rustc_copy_clone_marker]
pub enum Rule {
    any,
    soi,
    eoi,
    line,
    exclam,
    modulo,
    plus,
    qmark,
    star,
    dollar,
    capture,
    number,
    regex_token,
    str_token,
    token,
    quantifier,
    pat,
    pats_or_or,
    patseq,
    ruledef,
    quoted_name,
    plain_name,
    rule_name,
    rules,
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::clone::Clone for Rule {
    #[inline]
    fn clone(&self) -> Rule { { *self } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::marker::Copy for Rule { }
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::fmt::Debug for Rule {
    fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match (&*self,) {
            (&Rule::any,) => {
                let mut builder = __arg_0.debug_tuple("any");
                builder.finish()
            }
            (&Rule::soi,) => {
                let mut builder = __arg_0.debug_tuple("soi");
                builder.finish()
            }
            (&Rule::eoi,) => {
                let mut builder = __arg_0.debug_tuple("eoi");
                builder.finish()
            }
            (&Rule::line,) => {
                let mut builder = __arg_0.debug_tuple("line");
                builder.finish()
            }
            (&Rule::exclam,) => {
                let mut builder = __arg_0.debug_tuple("exclam");
                builder.finish()
            }
            (&Rule::modulo,) => {
                let mut builder = __arg_0.debug_tuple("modulo");
                builder.finish()
            }
            (&Rule::plus,) => {
                let mut builder = __arg_0.debug_tuple("plus");
                builder.finish()
            }
            (&Rule::qmark,) => {
                let mut builder = __arg_0.debug_tuple("qmark");
                builder.finish()
            }
            (&Rule::star,) => {
                let mut builder = __arg_0.debug_tuple("star");
                builder.finish()
            }
            (&Rule::dollar,) => {
                let mut builder = __arg_0.debug_tuple("dollar");
                builder.finish()
            }
            (&Rule::capture,) => {
                let mut builder = __arg_0.debug_tuple("capture");
                builder.finish()
            }
            (&Rule::number,) => {
                let mut builder = __arg_0.debug_tuple("number");
                builder.finish()
            }
            (&Rule::regex_token,) => {
                let mut builder = __arg_0.debug_tuple("regex_token");
                builder.finish()
            }
            (&Rule::str_token,) => {
                let mut builder = __arg_0.debug_tuple("str_token");
                builder.finish()
            }
            (&Rule::token,) => {
                let mut builder = __arg_0.debug_tuple("token");
                builder.finish()
            }
            (&Rule::quantifier,) => {
                let mut builder = __arg_0.debug_tuple("quantifier");
                builder.finish()
            }
            (&Rule::pat,) => {
                let mut builder = __arg_0.debug_tuple("pat");
                builder.finish()
            }
            (&Rule::pats_or_or,) => {
                let mut builder = __arg_0.debug_tuple("pats_or_or");
                builder.finish()
            }
            (&Rule::patseq,) => {
                let mut builder = __arg_0.debug_tuple("patseq");
                builder.finish()
            }
            (&Rule::ruledef,) => {
                let mut builder = __arg_0.debug_tuple("ruledef");
                builder.finish()
            }
            (&Rule::quoted_name,) => {
                let mut builder = __arg_0.debug_tuple("quoted_name");
                builder.finish()
            }
            (&Rule::plain_name,) => {
                let mut builder = __arg_0.debug_tuple("plain_name");
                builder.finish()
            }
            (&Rule::rule_name,) => {
                let mut builder = __arg_0.debug_tuple("rule_name");
                builder.finish()
            }
            (&Rule::rules,) => {
                let mut builder = __arg_0.debug_tuple("rules");
                builder.finish()
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Eq for Rule {
    #[inline]
    #[doc(hidden)]
    fn assert_receiver_is_total_eq(&self) -> () { { } }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::hash::Hash for Rule {
    fn hash<__H: ::std::hash::Hasher>(&self, __arg_0: &mut __H) -> () {
        match (&*self,) {
            _ => {
                ::std::hash::Hash::hash(&unsafe {
                                             ::std::intrinsics::discriminant_value(self)
                                         }, __arg_0)
            }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::Ord for Rule {
    #[inline]
    fn cmp(&self, __arg_0: &Rule) -> ::std::cmp::Ordering {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) {
                    _ => ::std::cmp::Ordering::Equal,
                }
            } else { __self_vi.cmp(&__arg_1_vi) }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialEq for Rule {
    #[inline]
    fn eq(&self, __arg_0: &Rule) -> bool {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) { _ => true, }
            } else { false }
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
#[allow(dead_code, non_camel_case_types)]
impl ::std::cmp::PartialOrd for Rule {
    #[inline]
    fn partial_cmp(&self, __arg_0: &Rule)
     -> ::std::option::Option<::std::cmp::Ordering> {
        {
            let __self_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*self) } as
                    isize;
            let __arg_1_vi =
                unsafe { ::std::intrinsics::discriminant_value(&*__arg_0) } as
                    isize;
            if true && __self_vi == __arg_1_vi {
                match (&*self, &*__arg_0) {
                    _ =>
                    ::std::option::Option::Some(::std::cmp::Ordering::Equal),
                }
            } else { __self_vi.partial_cmp(&__arg_1_vi) }
        }
    }
}
impl <'input, T: ::Input<'input>> Rdp<T> {
    pub fn new(input: T) -> Rdp<T> {
        Rdp{input: input,
            queue: <[_]>::into_vec(box []),
            queue_index: ::std::cell::Cell::new(0),
            failures: <[_]>::into_vec(box []),
            fail_pos: 0,
            stack: <[_]>::into_vec(box []),
            atomic: false,
            eoi_matched: false,}
    }
    #[allow(dead_code)]
    #[inline]
    pub fn comment(&mut self) -> bool { false }
    #[allow(dead_code)]
    #[inline]
    pub fn any(&mut self) -> bool {
        if self.end() {
            let pos = self.input.pos();
            self.track(Rule::any, pos);
            false
        } else {
            let next = self.input.pos() + 1;
            self.input.set_pos(next);
            true
        }
    }
    #[allow(dead_code)]
    #[inline]
    pub fn soi(&mut self) -> bool {
        let result = self.input.pos() == 0;
        if !result { let pos = self.input.pos(); self.track(Rule::soi, pos); }
        result
    }
    #[allow(dead_code)]
    #[inline]
    pub fn eoi(&mut self) -> bool {
        let result = self.end();
        if !result {
            let pos = self.input.pos();
            self.track(Rule::eoi, pos);
        } else { self.eoi_matched = true; }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn rules(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    (if (slf.newline() || slf.ruledef()) {
                         loop  {
                             let pos = slf.input().pos();
                             let len = slf.queue().len();
                             slf.skip();
                             if !(slf.newline() || slf.ruledef()) {
                                 slf.input_mut().set_pos(pos);
                                 slf.queue_mut().truncate(len);
                                 break
                             }
                         }
                         true
                     } else { false })
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::rules, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::rules, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn rule_name(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = { (slf.plain_name() || slf.quoted_name()) };
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::rule_name, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::rule_name, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn letter(&mut self) -> bool {
        let slf = self;
        let result =
            {
                {
                    ((slf.input_mut().match_range('a', 'z') ||
                          slf.input_mut().match_range('A', 'Z')) ||
                         slf.input_mut().match_string("_"))
                }
            };
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn plain_name(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let toggled = slf.is_atomic();
        if !toggled { slf.set_atomic(true); }
        let result =
            {
                {
                    {
                        {
                            (slf.try(false,
                                     |slf|
                                         {
                                             slf.letter() &&
                                                 ({
                                                      loop  {
                                                          if !((slf.letter()
                                                                    ||
                                                                    slf.input_mut().match_string("-"))
                                                                   ||
                                                                   slf.input_mut().match_range('0',
                                                                                               '9'))
                                                             {
                                                              break
                                                          }
                                                      }
                                                      true
                                                  })
                                         }))
                        }
                    }
                }
            };
        if !toggled { slf.set_atomic(false); }
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::plain_name, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            slf.track(Rule::plain_name, pos);
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn quoted_name(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let toggled = slf.is_atomic();
        if !toggled { slf.set_atomic(true); }
        let result =
            {
                {
                    {
                        {
                            {
                                (slf.try(false,
                                         |slf|
                                             {
                                                 (slf.try(false,
                                                          |slf|
                                                              {
                                                                  slf.input_mut().match_string("\'")
                                                                      &&
                                                                      ({
                                                                           loop 
                                                                                {
                                                                               if !(slf.try(false,
                                                                                            |slf|
                                                                                                {
                                                                                                    (slf.try(true,
                                                                                                             |slf|
                                                                                                                 {
                                                                                                                     !slf.input_mut().match_string("\'")
                                                                                                                 }))
                                                                                                        &&
                                                                                                        slf.any()
                                                                                                }))
                                                                                  {
                                                                                   break

                                                                               }
                                                                           }
                                                                           true
                                                                       })
                                                              })) &&
                                                     slf.input_mut().match_string("\'")
                                             }))
                            }
                        }
                    }
                }
            };
        if !toggled { slf.set_atomic(false); }
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::quoted_name, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            slf.track(Rule::quoted_name, pos);
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn newline(&mut self) -> bool {
        let slf = self;
        let result =
            {
                (slf.input_mut().match_string("\n") ||
                     slf.input_mut().match_string("\r\n"))
            };
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn whitespace(&mut self) -> bool {
        let slf = self;
        let result =
            {
                (slf.input_mut().match_string(" ") ||
                     slf.input_mut().match_string("\t"))
            };
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn ruledef(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            (slf.try(false,
                                     |slf|
                                         {
                                             if (slf.try(false,
                                                         |slf|
                                                             {
                                                                 if (slf.try(false,
                                                                             |slf|
                                                                                 {
                                                                                     if slf.rule_name()
                                                                                        {
                                                                                         let original_pos =
                                                                                             slf.input().pos();
                                                                                         let original_len =
                                                                                             slf.queue().len();
                                                                                         slf.skip();
                                                                                         let pos =
                                                                                             slf.input().pos();
                                                                                         let result =
                                                                                             slf.colon();
                                                                                         if slf.input().pos()
                                                                                                ==
                                                                                                pos
                                                                                                &&
                                                                                                !slf.eoi_matched()
                                                                                            {
                                                                                             slf.input_mut().set_pos(original_pos);
                                                                                             slf.queue_mut().truncate(original_len);
                                                                                         }
                                                                                         result
                                                                                     } else {
                                                                                         false
                                                                                     }
                                                                                 }))
                                                                    {
                                                                     let original_pos =
                                                                         slf.input().pos();
                                                                     let original_len =
                                                                         slf.queue().len();
                                                                     slf.skip();
                                                                     let pos =
                                                                         slf.input().pos();
                                                                     let result =
                                                                         slf.pats_or_or();
                                                                     if slf.input().pos()
                                                                            ==
                                                                            pos
                                                                            &&
                                                                            !slf.eoi_matched()
                                                                        {
                                                                         slf.input_mut().set_pos(original_pos);
                                                                         slf.queue_mut().truncate(original_len);
                                                                     }
                                                                     result
                                                                 } else {
                                                                     false
                                                                 }
                                                             })) {
                                                 let original_pos =
                                                     slf.input().pos();
                                                 let original_len =
                                                     slf.queue().len();
                                                 slf.skip();
                                                 let pos = slf.input().pos();
                                                 let result =
                                                     (slf.newline() ||
                                                          slf.eoi());
                                                 if slf.input().pos() == pos
                                                        && !slf.eoi_matched()
                                                    {
                                                     slf.input_mut().set_pos(original_pos);
                                                     slf.queue_mut().truncate(original_len);
                                                 }
                                                 result
                                             } else { false }
                                         }))
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::ruledef, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::ruledef, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn patseq(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                (if slf.pat() {
                     loop  {
                         let pos = slf.input().pos();
                         let len = slf.queue().len();
                         slf.skip();
                         if !slf.pat() {
                             slf.input_mut().set_pos(pos);
                             slf.queue_mut().truncate(len);
                             break
                         }
                     }
                     true
                 } else { false })
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::patseq, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::patseq, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn pats_or_or(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        (slf.try(false,
                                 |slf|
                                     {
                                         if slf.patseq() {
                                             let original_pos =
                                                 slf.input().pos();
                                             let original_len =
                                                 slf.queue().len();
                                             slf.skip();
                                             let pos = slf.input().pos();
                                             let result =
                                                 ({
                                                      let mut pos =
                                                          slf.input().pos();
                                                      let mut len =
                                                          slf.queue().len();
                                                      loop  {
                                                          if !(slf.try(false,
                                                                       |slf|
                                                                           {
                                                                               if slf.line()
                                                                                  {
                                                                                   let original_pos =
                                                                                       slf.input().pos();
                                                                                   let original_len =
                                                                                       slf.queue().len();
                                                                                   slf.skip();
                                                                                   let pos =
                                                                                       slf.input().pos();
                                                                                   let result =
                                                                                       slf.patseq();
                                                                                   if slf.input().pos()
                                                                                          ==
                                                                                          pos
                                                                                          &&
                                                                                          !slf.eoi_matched()
                                                                                      {
                                                                                       slf.input_mut().set_pos(original_pos);
                                                                                       slf.queue_mut().truncate(original_len);
                                                                                   }
                                                                                   result
                                                                               } else {
                                                                                   false
                                                                               }
                                                                           }))
                                                             {
                                                              slf.input_mut().set_pos(pos);
                                                              slf.queue_mut().truncate(len);
                                                              break
                                                          }
                                                          pos =
                                                              slf.input().pos();
                                                          len =
                                                              slf.queue().len();
                                                          slf.skip();
                                                      }
                                                      true
                                                  });
                                             if slf.input().pos() == pos &&
                                                    !slf.eoi_matched() {
                                                 slf.input_mut().set_pos(original_pos);
                                                 slf.queue_mut().truncate(original_len);
                                             }
                                             result
                                         } else { false }
                                     }))
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::pats_or_or, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::pats_or_or, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn pat(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            {
                                {
                                    {
                                        {
                                            (slf.try(false,
                                                     |slf|
                                                         {
                                                             if (slf.try(false,
                                                                         |slf|
                                                                             {
                                                                                 if ({
                                                                                         slf.capture();
                                                                                         true
                                                                                     })
                                                                                    {
                                                                                     let original_pos =
                                                                                         slf.input().pos();
                                                                                     let original_len =
                                                                                         slf.queue().len();
                                                                                     slf.skip();
                                                                                     let pos =
                                                                                         slf.input().pos();
                                                                                     let result =
                                                                                         ((slf.rule_name()
                                                                                               ||
                                                                                               slf.token())
                                                                                              ||
                                                                                              (slf.try(false,
                                                                                                       |slf|
                                                                                                           {
                                                                                                               if (slf.try(false,
                                                                                                                           |slf|
                                                                                                                               {
                                                                                                                                   if slf.input_mut().match_string("(")
                                                                                                                                      {
                                                                                                                                       let original_pos =
                                                                                                                                           slf.input().pos();
                                                                                                                                       let original_len =
                                                                                                                                           slf.queue().len();
                                                                                                                                       slf.skip();
                                                                                                                                       let pos =
                                                                                                                                           slf.input().pos();
                                                                                                                                       let result =
                                                                                                                                           slf.pats_or_or();
                                                                                                                                       if slf.input().pos()
                                                                                                                                              ==
                                                                                                                                              pos
                                                                                                                                              &&
                                                                                                                                              !slf.eoi_matched()
                                                                                                                                          {
                                                                                                                                           slf.input_mut().set_pos(original_pos);
                                                                                                                                           slf.queue_mut().truncate(original_len);
                                                                                                                                       }
                                                                                                                                       result
                                                                                                                                   } else {
                                                                                                                                       false
                                                                                                                                   }
                                                                                                                               }))
                                                                                                                  {
                                                                                                                   let original_pos =
                                                                                                                       slf.input().pos();
                                                                                                                   let original_len =
                                                                                                                       slf.queue().len();
                                                                                                                   slf.skip();
                                                                                                                   let pos =
                                                                                                                       slf.input().pos();
                                                                                                                   let result =
                                                                                                                       slf.input_mut().match_string(")");
                                                                                                                   if slf.input().pos()
                                                                                                                          ==
                                                                                                                          pos
                                                                                                                          &&
                                                                                                                          !slf.eoi_matched()
                                                                                                                      {
                                                                                                                       slf.input_mut().set_pos(original_pos);
                                                                                                                       slf.queue_mut().truncate(original_len);
                                                                                                                   }
                                                                                                                   result
                                                                                                               } else {
                                                                                                                   false
                                                                                                               }
                                                                                                           })));
                                                                                     if slf.input().pos()
                                                                                            ==
                                                                                            pos
                                                                                            &&
                                                                                            !slf.eoi_matched()
                                                                                        {
                                                                                         slf.input_mut().set_pos(original_pos);
                                                                                         slf.queue_mut().truncate(original_len);
                                                                                     }
                                                                                     result
                                                                                 } else {
                                                                                     false
                                                                                 }
                                                                             }))
                                                                {
                                                                 let original_pos =
                                                                     slf.input().pos();
                                                                 let original_len =
                                                                     slf.queue().len();
                                                                 slf.skip();
                                                                 let pos =
                                                                     slf.input().pos();
                                                                 let result =
                                                                     ({
                                                                          slf.quantifier();
                                                                          true
                                                                      });
                                                                 if slf.input().pos()
                                                                        == pos
                                                                        &&
                                                                        !slf.eoi_matched()
                                                                    {
                                                                     slf.input_mut().set_pos(original_pos);
                                                                     slf.queue_mut().truncate(original_len);
                                                                 }
                                                                 result
                                                             } else { false }
                                                         }))
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::pat, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::pat, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn quantifier(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            ((((slf.qmark() || slf.star()) || slf.plus()) ||
                                  slf.exclam()) || slf.modulo())
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::quantifier, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::quantifier, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn token(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = { (slf.str_token() || slf.regex_token()) };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::token, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::token, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn str_token(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            (slf.try(false,
                                     |slf|
                                         {
                                             if (slf.try(false,
                                                         |slf|
                                                             {
                                                                 if slf.input_mut().match_string("\"")
                                                                    {
                                                                     let original_pos =
                                                                         slf.input().pos();
                                                                     let original_len =
                                                                         slf.queue().len();
                                                                     slf.skip();
                                                                     let pos =
                                                                         slf.input().pos();
                                                                     let result =
                                                                         (slf.try(false,
                                                                                  |slf|
                                                                                      {
                                                                                          if (slf.try(true,
                                                                                                      |slf|
                                                                                                          {
                                                                                                              !slf.input_mut().match_string("\"")
                                                                                                          }))
                                                                                             {
                                                                                              let original_pos =
                                                                                                  slf.input().pos();
                                                                                              let original_len =
                                                                                                  slf.queue().len();
                                                                                              slf.skip();
                                                                                              let pos =
                                                                                                  slf.input().pos();
                                                                                              let result =
                                                                                                  slf.any();
                                                                                              if slf.input().pos()
                                                                                                     ==
                                                                                                     pos
                                                                                                     &&
                                                                                                     !slf.eoi_matched()
                                                                                                 {
                                                                                                  slf.input_mut().set_pos(original_pos);
                                                                                                  slf.queue_mut().truncate(original_len);
                                                                                              }
                                                                                              result
                                                                                          } else {
                                                                                              false
                                                                                          }
                                                                                      }));
                                                                     if slf.input().pos()
                                                                            ==
                                                                            pos
                                                                            &&
                                                                            !slf.eoi_matched()
                                                                        {
                                                                         slf.input_mut().set_pos(original_pos);
                                                                         slf.queue_mut().truncate(original_len);
                                                                     }
                                                                     result
                                                                 } else {
                                                                     false
                                                                 }
                                                             })) {
                                                 let original_pos =
                                                     slf.input().pos();
                                                 let original_len =
                                                     slf.queue().len();
                                                 slf.skip();
                                                 let pos = slf.input().pos();
                                                 let result =
                                                     slf.input_mut().match_string("\"");
                                                 if slf.input().pos() == pos
                                                        && !slf.eoi_matched()
                                                    {
                                                     slf.input_mut().set_pos(original_pos);
                                                     slf.queue_mut().truncate(original_len);
                                                 }
                                                 result
                                             } else { false }
                                         }))
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::str_token, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::str_token, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn regex_token(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            {
                                {
                                    (slf.try(false,
                                             |slf|
                                                 {
                                                     if (slf.try(false,
                                                                 |slf|
                                                                     {
                                                                         if slf.input_mut().match_string("r\"")
                                                                            {
                                                                             let original_pos =
                                                                                 slf.input().pos();
                                                                             let original_len =
                                                                                 slf.queue().len();
                                                                             slf.skip();
                                                                             let pos =
                                                                                 slf.input().pos();
                                                                             let result =
                                                                                 ((slf.try(false,
                                                                                           |slf|
                                                                                               {
                                                                                                   if slf.input_mut().match_string("\\")
                                                                                                      {
                                                                                                       let original_pos =
                                                                                                           slf.input().pos();
                                                                                                       let original_len =
                                                                                                           slf.queue().len();
                                                                                                       slf.skip();
                                                                                                       let pos =
                                                                                                           slf.input().pos();
                                                                                                       let result =
                                                                                                           slf.any();
                                                                                                       if slf.input().pos()
                                                                                                              ==
                                                                                                              pos
                                                                                                              &&
                                                                                                              !slf.eoi_matched()
                                                                                                          {
                                                                                                           slf.input_mut().set_pos(original_pos);
                                                                                                           slf.queue_mut().truncate(original_len);
                                                                                                       }
                                                                                                       result
                                                                                                   } else {
                                                                                                       false
                                                                                                   }
                                                                                               }))
                                                                                      ||
                                                                                      (slf.try(false,
                                                                                               |slf|
                                                                                                   {
                                                                                                       if (slf.try(true,
                                                                                                                   |slf|
                                                                                                                       {
                                                                                                                           !slf.input_mut().match_string("\"")
                                                                                                                       }))
                                                                                                          {
                                                                                                           let original_pos =
                                                                                                               slf.input().pos();
                                                                                                           let original_len =
                                                                                                               slf.queue().len();
                                                                                                           slf.skip();
                                                                                                           let pos =
                                                                                                               slf.input().pos();
                                                                                                           let result =
                                                                                                               slf.any();
                                                                                                           if slf.input().pos()
                                                                                                                  ==
                                                                                                                  pos
                                                                                                                  &&
                                                                                                                  !slf.eoi_matched()
                                                                                                              {
                                                                                                               slf.input_mut().set_pos(original_pos);
                                                                                                               slf.queue_mut().truncate(original_len);
                                                                                                           }
                                                                                                           result
                                                                                                       } else {
                                                                                                           false
                                                                                                       }
                                                                                                   })));
                                                                             if slf.input().pos()
                                                                                    ==
                                                                                    pos
                                                                                    &&
                                                                                    !slf.eoi_matched()
                                                                                {
                                                                                 slf.input_mut().set_pos(original_pos);
                                                                                 slf.queue_mut().truncate(original_len);
                                                                             }
                                                                             result
                                                                         } else {
                                                                             false
                                                                         }
                                                                     })) {
                                                         let original_pos =
                                                             slf.input().pos();
                                                         let original_len =
                                                             slf.queue().len();
                                                         slf.skip();
                                                         let pos =
                                                             slf.input().pos();
                                                         let result =
                                                             slf.input_mut().match_string("\"");
                                                         if slf.input().pos()
                                                                == pos &&
                                                                !slf.eoi_matched()
                                                            {
                                                             slf.input_mut().set_pos(original_pos);
                                                             slf.queue_mut().truncate(original_len);
                                                         }
                                                         result
                                                     } else { false }
                                                 }))
                                }
                            }
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token =
                Token{rule: Rule::regex_token, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::regex_token, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn number(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                (if slf.input_mut().match_range('0', '9') {
                     loop  {
                         let pos = slf.input().pos();
                         let len = slf.queue().len();
                         slf.skip();
                         if !slf.input_mut().match_range('0', '9') {
                             slf.input_mut().set_pos(pos);
                             slf.queue_mut().truncate(len);
                             break
                         }
                     }
                     true
                 } else { false })
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::number, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::number, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn capture(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result =
            {
                {
                    {
                        {
                            (slf.try(false,
                                     |slf|
                                         {
                                             if slf.dollar() {
                                                 let original_pos =
                                                     slf.input().pos();
                                                 let original_len =
                                                     slf.queue().len();
                                                 slf.skip();
                                                 let pos = slf.input().pos();
                                                 let result =
                                                     ({
                                                          (({
                                                                let mut pos =
                                                                    slf.input().pos();
                                                                let mut len =
                                                                    slf.queue().len();
                                                                loop  {
                                                                    if !slf.dollar()
                                                                       {
                                                                        slf.input_mut().set_pos(pos);
                                                                        slf.queue_mut().truncate(len);
                                                                        break
                                                                    }
                                                                    pos =
                                                                        slf.input().pos();
                                                                    len =
                                                                        slf.queue().len();
                                                                    slf.skip();
                                                                }
                                                                true
                                                            }) ||
                                                               slf.number());
                                                          true
                                                      });
                                                 if slf.input().pos() == pos
                                                        && !slf.eoi_matched()
                                                    {
                                                     slf.input_mut().set_pos(original_pos);
                                                     slf.queue_mut().truncate(original_len);
                                                 }
                                                 result
                                             } else { false }
                                         }))
                        }
                    }
                }
            };
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::capture, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::capture, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn dollar(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("$");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::dollar, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::dollar, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn star(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("*");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::star, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::star, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn qmark(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("?");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::qmark, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::qmark, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn plus(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("+");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::plus, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::plus, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn modulo(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("%");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::modulo, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::modulo, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn exclam(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("!");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::exclam, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::exclam, pos);
            }
        }
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn colon(&mut self) -> bool {
        let slf = self;
        let result = slf.input_mut().match_string(":");
        result
    }
    #[allow(unused_parens, unused_variables)]
    #[inline]
    pub fn line(&mut self) -> bool {
        let slf = self;
        let pos = slf.input().pos();
        let len = slf.queue().len();
        let tracked_len_pos = slf.tracked_len_pos();
        let result = slf.input_mut().match_string("|");
        if result {
            let new_pos = slf.input().pos();
            let token = Token{rule: Rule::line, start: pos, end: new_pos,};
            slf.queue_mut().insert(len, token);
        } else {
            slf.queue_mut().truncate(len);
            if slf.tracked_len_pos() == tracked_len_pos {
                slf.track(Rule::line, pos);
            }
        }
        result
    }
    #[allow(dead_code)]
    #[inline]
    pub fn try<F>(&mut self, revert: bool, rule: F) -> bool where
     F: FnOnce(&mut Self) -> bool {
        let pos = self.input().pos();
        let len = self.queue().len();
        let result = rule(self);
        if revert || !result {
            self.input_mut().set_pos(pos);
            self.queue_mut().truncate(len);
        }
        result
    }
    #[allow(dead_code)]
    #[inline]
    pub fn prec_climb<F,
                      G>(&mut self, pos: usize, left: usize, min_prec: u8,
                         last_op: Option<(Option<Rule>, u8, bool)>,
                         primary: &mut F, climb: &mut G)
     -> (Option<(Option<Rule>, u8, bool)>, Option<usize>) where
     F: FnMut(&mut Self) -> bool, G: FnMut(&mut Self) ->
     Option<(Option<Rule>, u8, bool)> {
        let mut op = if last_op.is_some() { last_op } else { climb(self) };
        let mut last_right = None;
        while let Some((rule, prec, _)) = op {
            if prec >= min_prec {
                let mut new_pos = self.input().pos();
                let mut right = self.input().pos();
                let queue_pos = self.queue().len();
                if !primary(self) {
                    let last_pos = self.queue().get(pos).unwrap().end;
                    self.input_mut().set_pos(last_pos);
                    self.queue_mut().truncate(pos + 1);
                    break
                }
                if let Some(token) = self.queue().get(queue_pos) {
                    new_pos = token.start;
                    right = token.end;
                }
                op = climb(self);
                while let Some((_, new_prec, right_assoc)) = op {
                    if new_prec > prec || right_assoc && new_prec == prec {
                        let (new_op, new_lr) =
                            self.prec_climb(queue_pos, new_pos, new_prec, op,
                                            primary, climb);
                        op = new_op;
                        last_right = new_lr;
                    } else { break  }
                }
                if let Some(pos) = last_right {
                    right = ::std::cmp::max(pos, right);
                } else { last_right = Some(right); }
                if let Some(rule) = rule {
                    let token = Token{rule: rule, start: left, end: right,};
                    self.queue_mut().insert(pos, token);
                }
            } else { return (op, last_right) }
        }
        (op, last_right)
    }
    #[inline]
    pub fn main(&self) -> HashMap<String, GrammarRule> {
        if let Some(result) =
               {
                   if let Some(token) = self.queue().get(self.queue_index()) {
                       if token.rule == Rule::rules {
                           self.inc_queue_index();
                           { let map = self._rules(); Some({ map }) }
                       } else { None }
                   } else { None }
               } {
            result
        } else {
            let next =
                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                          token.rule).fold("".to_owned(),
                                                                                           |acc,
                                                                                            rule|
                                                                                               acc
                                                                                                   +
                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                           &'static [&'static str]
                                                                                                                                                           =
                                                                                                                                                        &["",
                                                                                                                                                          ", "];
                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                },
                                                                                                                                                &match (&rule,)
                                                                                                                                                     {
                                                                                                                                                     (__arg0,)
                                                                                                                                                     =>
                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                 })));
            {
                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                         static __STATIC_FMTSTR:
                                                                                &'static [&'static str]
                                                                                =
                                                                             &["no pattern matched in ",
                                                                               "; failed at [",
                                                                               "...]"];
                                                                         __STATIC_FMTSTR
                                                                     },
                                                                     &match (&"main",
                                                                             &next)
                                                                          {
                                                                          (__arg0,
                                                                           __arg1)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Display::fmt),
                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      }),
                                      {
                                          static _FILE_LINE_COL:
                                                 (&'static str, u32, u32) =
                                              ("src/main.rs", 52u32, 0u32);
                                          &_FILE_LINE_COL
                                      })
            }
        }
    }
    #[inline]
    pub fn _rules(&self) -> HashMap<String, GrammarRule> {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::ruledef {
                               self.inc_queue_index();
                               {
                                   let rule = self._ruledef();
                                   {
                                       let mut map = self._rules();
                                       Some({
                                                map.insert(rule.name.clone(),
                                                           rule);
                                                map
                                            })
                                   }
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                if let Some(result) = { Some({ HashMap::new() }) } {
                    result
                } else {
                    let next =
                        self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                  token.rule).fold("".to_owned(),
                                                                                                   |acc,
                                                                                                    rule|
                                                                                                       acc
                                                                                                           +
                                                                                                           &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                            static __STATIC_FMTSTR:
                                                                                                                                                                   &'static [&'static str]
                                                                                                                                                                   =
                                                                                                                                                                &["",
                                                                                                                                                                  ", "];
                                                                                                                                                            __STATIC_FMTSTR
                                                                                                                                                        },
                                                                                                                                                        &match (&rule,)
                                                                                                                                                             {
                                                                                                                                                             (__arg0,)
                                                                                                                                                             =>
                                                                                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                         })));
                    {
                        ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                 static __STATIC_FMTSTR:
                                                                                        &'static [&'static str]
                                                                                        =
                                                                                     &["no pattern matched in ",
                                                                                       "; failed at [",
                                                                                       "...]"];
                                                                                 __STATIC_FMTSTR
                                                                             },
                                                                             &match (&"_rules",
                                                                                     &next)
                                                                                  {
                                                                                  (__arg0,
                                                                                   __arg1)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                               ::std::fmt::Display::fmt),
                                                                                   ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }),
                                              {
                                                  static _FILE_LINE_COL:
                                                         (&'static str, u32,
                                                          u32) =
                                                      ("src/main.rs", 52u32,
                                                       0u32);
                                                  &_FILE_LINE_COL
                                              })
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _ruledef(&self) -> GrammarRule {
        if let Some(result) =
               {
                   if let Some(token) = self.queue().get(self.queue_index()) {
                       if token.rule == Rule::rule_name {
                           self.inc_queue_index();
                           {
                               let name = self._rule_name();
                               {
                                   if let Some(token) =
                                          self.queue().get(self.queue_index())
                                          {
                                       if token.rule == Rule::pats_or_or {
                                           self.inc_queue_index();
                                           {
                                               let pat = self._pats_or_or();
                                               Some({
                                                        GrammarRule{name,
                                                                    pat,
                                                                    nof_captures:
                                                                        1,}
                                                    })
                                           }
                                       } else { None }
                                   } else { None }
                               }
                           }
                       } else { None }
                   } else { None }
               } {
            result
        } else {
            let next =
                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                          token.rule).fold("".to_owned(),
                                                                                           |acc,
                                                                                            rule|
                                                                                               acc
                                                                                                   +
                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                           &'static [&'static str]
                                                                                                                                                           =
                                                                                                                                                        &["",
                                                                                                                                                          ", "];
                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                },
                                                                                                                                                &match (&rule,)
                                                                                                                                                     {
                                                                                                                                                     (__arg0,)
                                                                                                                                                     =>
                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                 })));
            {
                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                         static __STATIC_FMTSTR:
                                                                                &'static [&'static str]
                                                                                =
                                                                             &["no pattern matched in ",
                                                                               "; failed at [",
                                                                               "...]"];
                                                                         __STATIC_FMTSTR
                                                                     },
                                                                     &match (&"_ruledef",
                                                                             &next)
                                                                          {
                                                                          (__arg0,
                                                                           __arg1)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Display::fmt),
                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      }),
                                      {
                                          static _FILE_LINE_COL:
                                                 (&'static str, u32, u32) =
                                              ("src/main.rs", 52u32, 0u32);
                                          &_FILE_LINE_COL
                                      })
            }
        }
    }
    #[inline]
    pub fn _pats_or_or(&self) -> Pat {
        if let Some(result) =
               {
                   let mut rev_pats = self.__pats_or_or();
                   Some({
                            let has_one = rev_pats.len() == 1;
                            if has_one {
                                rev_pats.pop().unwrap()
                            } else {
                                rev_pats.reverse();
                                Pat::AnyOf(rev_pats)
                            }
                        })
               } {
            result
        } else {
            let next =
                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                          token.rule).fold("".to_owned(),
                                                                                           |acc,
                                                                                            rule|
                                                                                               acc
                                                                                                   +
                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                           &'static [&'static str]
                                                                                                                                                           =
                                                                                                                                                        &["",
                                                                                                                                                          ", "];
                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                },
                                                                                                                                                &match (&rule,)
                                                                                                                                                     {
                                                                                                                                                     (__arg0,)
                                                                                                                                                     =>
                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                 })));
            {
                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                         static __STATIC_FMTSTR:
                                                                                &'static [&'static str]
                                                                                =
                                                                             &["no pattern matched in ",
                                                                               "; failed at [",
                                                                               "...]"];
                                                                         __STATIC_FMTSTR
                                                                     },
                                                                     &match (&"_pats_or_or",
                                                                             &next)
                                                                          {
                                                                          (__arg0,
                                                                           __arg1)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Display::fmt),
                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      }),
                                      {
                                          static _FILE_LINE_COL:
                                                 (&'static str, u32, u32) =
                                              ("src/main.rs", 52u32, 0u32);
                                          &_FILE_LINE_COL
                                      })
            }
        }
    }
    #[inline]
    pub fn __pats_or_or(&self) -> Vec<Pat> {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::patseq {
                               self.inc_queue_index();
                               {
                                   let pat = self._patseq();
                                   {
                                       if let Some(token) =
                                              self.queue().get(self.queue_index())
                                              {
                                           if token.rule == Rule::line {
                                               self.inc_queue_index();
                                               {
                                                   let mut tail =
                                                       self.__pats_or_or();
                                                   Some({
                                                            tail.push(pat);
                                                            tail
                                                        })
                                               }
                                           } else { None }
                                       } else { None }
                                   }
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                if let Some(result) =
                       {
                           if let Some(token) =
                                  self.queue().get(self.queue_index()) {
                               if token.rule == Rule::patseq {
                                   self.inc_queue_index();
                                   {
                                       let pat = self._patseq();
                                       Some({ <[_]>::into_vec(box [pat]) })
                                   }
                               } else { None }
                           } else { None }
                       } {
                    result
                } else {
                    let next =
                        self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                  token.rule).fold("".to_owned(),
                                                                                                   |acc,
                                                                                                    rule|
                                                                                                       acc
                                                                                                           +
                                                                                                           &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                            static __STATIC_FMTSTR:
                                                                                                                                                                   &'static [&'static str]
                                                                                                                                                                   =
                                                                                                                                                                &["",
                                                                                                                                                                  ", "];
                                                                                                                                                            __STATIC_FMTSTR
                                                                                                                                                        },
                                                                                                                                                        &match (&rule,)
                                                                                                                                                             {
                                                                                                                                                             (__arg0,)
                                                                                                                                                             =>
                                                                                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                         })));
                    {
                        ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                 static __STATIC_FMTSTR:
                                                                                        &'static [&'static str]
                                                                                        =
                                                                                     &["no pattern matched in ",
                                                                                       "; failed at [",
                                                                                       "...]"];
                                                                                 __STATIC_FMTSTR
                                                                             },
                                                                             &match (&"__pats_or_or",
                                                                                     &next)
                                                                                  {
                                                                                  (__arg0,
                                                                                   __arg1)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                               ::std::fmt::Display::fmt),
                                                                                   ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }),
                                              {
                                                  static _FILE_LINE_COL:
                                                         (&'static str, u32,
                                                          u32) =
                                                      ("src/main.rs", 52u32,
                                                       0u32);
                                                  &_FILE_LINE_COL
                                              })
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _patseq(&self) -> Pat {
        if let Some(result) =
               {
                   let mut rev_pats = self.__patseq();
                   Some({
                            let has_one = rev_pats.len() == 1;
                            if has_one {
                                rev_pats.pop().unwrap()
                            } else { rev_pats.reverse(); Pat::Seq(rev_pats) }
                        })
               } {
            result
        } else {
            let next =
                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                          token.rule).fold("".to_owned(),
                                                                                           |acc,
                                                                                            rule|
                                                                                               acc
                                                                                                   +
                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                           &'static [&'static str]
                                                                                                                                                           =
                                                                                                                                                        &["",
                                                                                                                                                          ", "];
                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                },
                                                                                                                                                &match (&rule,)
                                                                                                                                                     {
                                                                                                                                                     (__arg0,)
                                                                                                                                                     =>
                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                 })));
            {
                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                         static __STATIC_FMTSTR:
                                                                                &'static [&'static str]
                                                                                =
                                                                             &["no pattern matched in ",
                                                                               "; failed at [",
                                                                               "...]"];
                                                                         __STATIC_FMTSTR
                                                                     },
                                                                     &match (&"_patseq",
                                                                             &next)
                                                                          {
                                                                          (__arg0,
                                                                           __arg1)
                                                                          =>
                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                       ::std::fmt::Display::fmt),
                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                       ::std::fmt::Display::fmt)],
                                                                      }),
                                      {
                                          static _FILE_LINE_COL:
                                                 (&'static str, u32, u32) =
                                              ("src/main.rs", 52u32, 0u32);
                                          &_FILE_LINE_COL
                                      })
            }
        }
    }
    #[inline]
    pub fn __patseq(&self) -> Vec<Pat> {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::pat {
                               self.inc_queue_index();
                               {
                                   let head = self._pat();
                                   {
                                       let mut rev_pats = self.__patseq();
                                       Some({ rev_pats.push(head); rev_pats })
                                   }
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                if let Some(result) = { Some({ Vec::new() }) } {
                    result
                } else {
                    let next =
                        self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                  token.rule).fold("".to_owned(),
                                                                                                   |acc,
                                                                                                    rule|
                                                                                                       acc
                                                                                                           +
                                                                                                           &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                            static __STATIC_FMTSTR:
                                                                                                                                                                   &'static [&'static str]
                                                                                                                                                                   =
                                                                                                                                                                &["",
                                                                                                                                                                  ", "];
                                                                                                                                                            __STATIC_FMTSTR
                                                                                                                                                        },
                                                                                                                                                        &match (&rule,)
                                                                                                                                                             {
                                                                                                                                                             (__arg0,)
                                                                                                                                                             =>
                                                                                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                         })));
                    {
                        ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                 static __STATIC_FMTSTR:
                                                                                        &'static [&'static str]
                                                                                        =
                                                                                     &["no pattern matched in ",
                                                                                       "; failed at [",
                                                                                       "...]"];
                                                                                 __STATIC_FMTSTR
                                                                             },
                                                                             &match (&"__patseq",
                                                                                     &next)
                                                                                  {
                                                                                  (__arg0,
                                                                                   __arg1)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                               ::std::fmt::Display::fmt),
                                                                                   ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }),
                                              {
                                                  static _FILE_LINE_COL:
                                                         (&'static str, u32,
                                                          u32) =
                                                      ("src/main.rs", 52u32,
                                                       0u32);
                                                  &_FILE_LINE_COL
                                              })
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _pat(&self) -> Pat {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       let capture = self._capture();
                       {
                           let pat = self._pat();
                           {
                               let quantifier = self._quantifier();
                               Some({
                                        let mut pat =
                                            if let Some(quantifier) =
                                                   quantifier {
                                                match quantifier {
                                                    Quantifier::Opt =>
                                                    Pat::Opt(Box::new(pat)),
                                                    Quantifier::ZeroPlus =>
                                                    Pat::ZeroPlus(Box::new(pat)),
                                                    Quantifier::OnePlus =>
                                                    Pat::OnePlus(Box::new(pat)),
                                                    Quantifier::Loop =>
                                                    Pat::Loop(Box::new(pat)),
                                                    Quantifier::BreakOnToken
                                                    => {
                                                        if let Pat::Token(token)
                                                               = pat {
                                                            Pat::BreakOnToken(token)
                                                        } else {
                                                            {
                                                                ::rt::begin_panic("BreakOnToken put on non-token pattern",
                                                                                  {
                                                                                      static _FILE_LINE_COL:
                                                                                             (&'static str,
                                                                                              u32,
                                                                                              u32)
                                                                                             =
                                                                                          ("src/main.rs",
                                                                                           163u32,
                                                                                           32u32);
                                                                                      &_FILE_LINE_COL
                                                                                  })
                                                            };
                                                        }
                                                    }
                                                }
                                            } else { pat };
                                        if let Some(cap) = capture {
                                            Pat::Cap(cap, Box::new(pat))
                                        } else { pat }
                                    })
                           }
                       }
                   } {
                result
            } else {
                self.set_queue_index(index);
                if let Some(result) =
                       {
                           if let Some(token) =
                                  self.queue().get(self.queue_index()) {
                               if token.rule == Rule::rule_name {
                                   self.inc_queue_index();
                                   {
                                       let name = self._rule_name();
                                       Some({ Pat::Rule(name) })
                                   }
                               } else { None }
                           } else { None }
                       } {
                    result
                } else {
                    let next =
                        self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                  token.rule).fold("".to_owned(),
                                                                                                   |acc,
                                                                                                    rule|
                                                                                                       acc
                                                                                                           +
                                                                                                           &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                            static __STATIC_FMTSTR:
                                                                                                                                                                   &'static [&'static str]
                                                                                                                                                                   =
                                                                                                                                                                &["",
                                                                                                                                                                  ", "];
                                                                                                                                                            __STATIC_FMTSTR
                                                                                                                                                        },
                                                                                                                                                        &match (&rule,)
                                                                                                                                                             {
                                                                                                                                                             (__arg0,)
                                                                                                                                                             =>
                                                                                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                         })));
                    {
                        ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                 static __STATIC_FMTSTR:
                                                                                        &'static [&'static str]
                                                                                        =
                                                                                     &["no pattern matched in ",
                                                                                       "; failed at [",
                                                                                       "...]"];
                                                                                 __STATIC_FMTSTR
                                                                             },
                                                                             &match (&"_pat",
                                                                                     &next)
                                                                                  {
                                                                                  (__arg0,
                                                                                   __arg1)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                               ::std::fmt::Display::fmt),
                                                                                   ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }),
                                              {
                                                  static _FILE_LINE_COL:
                                                         (&'static str, u32,
                                                          u32) =
                                                      ("src/main.rs", 52u32,
                                                       0u32);
                                                  &_FILE_LINE_COL
                                              })
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _dollars(&self) -> usize {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::dollar {
                               self.inc_queue_index();
                               {
                                   let nof_dollars = self._dollars();
                                   Some({ nof_dollars + 1 })
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                if let Some(result) = { Some({ 0 }) } {
                    result
                } else {
                    let next =
                        self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                  token.rule).fold("".to_owned(),
                                                                                                   |acc,
                                                                                                    rule|
                                                                                                       acc
                                                                                                           +
                                                                                                           &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                            static __STATIC_FMTSTR:
                                                                                                                                                                   &'static [&'static str]
                                                                                                                                                                   =
                                                                                                                                                                &["",
                                                                                                                                                                  ", "];
                                                                                                                                                            __STATIC_FMTSTR
                                                                                                                                                        },
                                                                                                                                                        &match (&rule,)
                                                                                                                                                             {
                                                                                                                                                             (__arg0,)
                                                                                                                                                             =>
                                                                                                                                                             [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                          ::std::fmt::Debug::fmt)],
                                                                                                                                                         })));
                    {
                        ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                 static __STATIC_FMTSTR:
                                                                                        &'static [&'static str]
                                                                                        =
                                                                                     &["no pattern matched in ",
                                                                                       "; failed at [",
                                                                                       "...]"];
                                                                                 __STATIC_FMTSTR
                                                                             },
                                                                             &match (&"_dollars",
                                                                                     &next)
                                                                                  {
                                                                                  (__arg0,
                                                                                   __arg1)
                                                                                  =>
                                                                                  [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                               ::std::fmt::Display::fmt),
                                                                                   ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                               ::std::fmt::Display::fmt)],
                                                                              }),
                                              {
                                                  static _FILE_LINE_COL:
                                                         (&'static str, u32,
                                                          u32) =
                                                      ("src/main.rs", 52u32,
                                                       0u32);
                                                  &_FILE_LINE_COL
                                              })
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _capture(&self) -> Option<Capture> {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::capture {
                               self.inc_queue_index();
                               {
                                   if let Some(token) =
                                          self.queue().get(self.queue_index())
                                          {
                                       if token.rule == Rule::dollar {
                                           self.inc_queue_index();
                                           {
                                               if let Some(token) =
                                                      self.queue().get(self.queue_index())
                                                      {
                                                   if token.rule ==
                                                          Rule::number {
                                                       let num =
                                                           self.input().slice(token.start,
                                                                              token.end);
                                                       self.inc_queue_index();
                                                       Some({
                                                                Some(Capture::Named(num.parse().unwrap_or(0)))
                                                            })
                                                   } else { None }
                                               } else { None }
                                           }
                                       } else { None }
                                   } else { None }
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                {
                    let index = self.queue_index();
                    if let Some(result) =
                           {
                               if let Some(token) =
                                      self.queue().get(self.queue_index()) {
                                   if token.rule == Rule::capture {
                                       self.inc_queue_index();
                                       {
                                           if let Some(token) =
                                                  self.queue().get(self.queue_index())
                                                  {
                                               if token.rule == Rule::dollar {
                                                   self.inc_queue_index();
                                                   {
                                                       let nof_dollars =
                                                           self._dollars();
                                                       Some({
                                                                if nof_dollars
                                                                       > 0 {
                                                                    Some(Capture::Unnamed)
                                                                } else {
                                                                    Some(Capture::Shared(nof_dollars
                                                                                             +
                                                                                             1))
                                                                }
                                                            })
                                                   }
                                               } else { None }
                                           } else { None }
                                       }
                                   } else { None }
                               } else { None }
                           } {
                        result
                    } else {
                        self.set_queue_index(index);
                        if let Some(result) = { Some({ None }) } {
                            result
                        } else {
                            let next =
                                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                          token.rule).fold("".to_owned(),
                                                                                                           |acc,
                                                                                                            rule|
                                                                                                               acc
                                                                                                                   +
                                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                                           &'static [&'static str]
                                                                                                                                                                           =
                                                                                                                                                                        &["",
                                                                                                                                                                          ", "];
                                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                                },
                                                                                                                                                                &match (&rule,)
                                                                                                                                                                     {
                                                                                                                                                                     (__arg0,)
                                                                                                                                                                     =>
                                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                                 })));
                            {
                                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                         static __STATIC_FMTSTR:
                                                                                                &'static [&'static str]
                                                                                                =
                                                                                             &["no pattern matched in ",
                                                                                               "; failed at [",
                                                                                               "...]"];
                                                                                         __STATIC_FMTSTR
                                                                                     },
                                                                                     &match (&"_capture",
                                                                                             &next)
                                                                                          {
                                                                                          (__arg0,
                                                                                           __arg1)
                                                                                          =>
                                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                       ::std::fmt::Display::fmt),
                                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                      }),
                                                      {
                                                          static _FILE_LINE_COL:
                                                                 (&'static str,
                                                                  u32, u32) =
                                                              ("src/main.rs",
                                                               52u32, 0u32);
                                                          &_FILE_LINE_COL
                                                      })
                            }
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _quantifier(&self) -> Option<Quantifier> {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::quantifier {
                               self.inc_queue_index();
                               {
                                   let quantifier = self._quantifier();
                                   Some({ quantifier })
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                {
                    let index = self.queue_index();
                    if let Some(result) =
                           {
                               if let Some(token) =
                                      self.queue().get(self.queue_index()) {
                                   if token.rule == Rule::qmark {
                                       self.inc_queue_index();
                                       Some({ Some(Quantifier::Opt) })
                                   } else { None }
                               } else { None }
                           } {
                        result
                    } else {
                        self.set_queue_index(index);
                        {
                            let index = self.queue_index();
                            if let Some(result) =
                                   {
                                       if let Some(token) =
                                              self.queue().get(self.queue_index())
                                              {
                                           if token.rule == Rule::star {
                                               self.inc_queue_index();
                                               Some({
                                                        Some(Quantifier::ZeroPlus)
                                                    })
                                           } else { None }
                                       } else { None }
                                   } {
                                result
                            } else {
                                self.set_queue_index(index);
                                {
                                    let index = self.queue_index();
                                    if let Some(result) =
                                           {
                                               if let Some(token) =
                                                      self.queue().get(self.queue_index())
                                                      {
                                                   if token.rule == Rule::plus
                                                      {
                                                       self.inc_queue_index();
                                                       Some({
                                                                Some(Quantifier::OnePlus)
                                                            })
                                                   } else { None }
                                               } else { None }
                                           } {
                                        result
                                    } else {
                                        self.set_queue_index(index);
                                        {
                                            let index = self.queue_index();
                                            if let Some(result) =
                                                   {
                                                       if let Some(token) =
                                                              self.queue().get(self.queue_index())
                                                              {
                                                           if token.rule ==
                                                                  Rule::exclam
                                                              {
                                                               self.inc_queue_index();
                                                               Some({
                                                                        Some(Quantifier::BreakOnToken)
                                                                    })
                                                           } else { None }
                                                       } else { None }
                                                   } {
                                                result
                                            } else {
                                                self.set_queue_index(index);
                                                {
                                                    let index =
                                                        self.queue_index();
                                                    if let Some(result) =
                                                           {
                                                               if let Some(token)
                                                                      =
                                                                      self.queue().get(self.queue_index())
                                                                      {
                                                                   if token.rule
                                                                          ==
                                                                          Rule::modulo
                                                                      {
                                                                       self.inc_queue_index();
                                                                       Some({
                                                                                Some(Quantifier::Loop)
                                                                            })
                                                                   } else {
                                                                       None
                                                                   }
                                                               } else { None }
                                                           } {
                                                        result
                                                    } else {
                                                        self.set_queue_index(index);
                                                        if let Some(result) =
                                                               {
                                                                   Some({
                                                                            None
                                                                        })
                                                               } {
                                                            result
                                                        } else {
                                                            let next =
                                                                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                                                          token.rule).fold("".to_owned(),
                                                                                                                                           |acc,
                                                                                                                                            rule|
                                                                                                                                               acc
                                                                                                                                                   +
                                                                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                                                                           &'static [&'static str]
                                                                                                                                                                                                           =
                                                                                                                                                                                                        &["",
                                                                                                                                                                                                          ", "];
                                                                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                                                                },
                                                                                                                                                                                                &match (&rule,)
                                                                                                                                                                                                     {
                                                                                                                                                                                                     (__arg0,)
                                                                                                                                                                                                     =>
                                                                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                                                                 })));
                                                            {
                                                                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                                                         static __STATIC_FMTSTR:
                                                                                                                                &'static [&'static str]
                                                                                                                                =
                                                                                                                             &["no pattern matched in ",
                                                                                                                               "; failed at [",
                                                                                                                               "...]"];
                                                                                                                         __STATIC_FMTSTR
                                                                                                                     },
                                                                                                                     &match (&"_quantifier",
                                                                                                                             &next)
                                                                                                                          {
                                                                                                                          (__arg0,
                                                                                                                           __arg1)
                                                                                                                          =>
                                                                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                       ::std::fmt::Display::fmt),
                                                                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                                                      }),
                                                                                      {
                                                                                          static _FILE_LINE_COL:
                                                                                                 (&'static str,
                                                                                                  u32,
                                                                                                  u32)
                                                                                                 =
                                                                                              ("src/main.rs",
                                                                                               52u32,
                                                                                               0u32);
                                                                                          &_FILE_LINE_COL
                                                                                      })
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    #[inline]
    pub fn _rule_name(&self) -> String {
        {
            let index = self.queue_index();
            if let Some(result) =
                   {
                       if let Some(token) =
                              self.queue().get(self.queue_index()) {
                           if token.rule == Rule::rule_name {
                               self.inc_queue_index();
                               {
                                   let rule = self._rule_name();
                                   Some({ rule })
                               }
                           } else { None }
                       } else { None }
                   } {
                result
            } else {
                self.set_queue_index(index);
                {
                    let index = self.queue_index();
                    if let Some(result) =
                           {
                               if let Some(token) =
                                      self.queue().get(self.queue_index()) {
                                   if token.rule == Rule::plain_name {
                                       let rule =
                                           self.input().slice(token.start,
                                                              token.end);
                                       self.inc_queue_index();
                                       Some({ rule.to_string() })
                                   } else { None }
                               } else { None }
                           } {
                        result
                    } else {
                        self.set_queue_index(index);
                        if let Some(result) =
                               {
                                   if let Some(token) =
                                          self.queue().get(self.queue_index())
                                          {
                                       if token.rule == Rule::quoted_name {
                                           let rule =
                                               self.input().slice(token.start,
                                                                  token.end);
                                           self.inc_queue_index();
                                           Some({
                                                    let end_quote =
                                                        rule.len() - 1;
                                                    (&rule[1..end_quote]).to_string()
                                                })
                                       } else { None }
                                   } else { None }
                               } {
                            result
                        } else {
                            let next =
                                self.queue()[self.queue_index()..].iter().take(3).map(|token|
                                                                                          token.rule).fold("".to_owned(),
                                                                                                           |acc,
                                                                                                            rule|
                                                                                                               acc
                                                                                                                   +
                                                                                                                   &::fmt::format(::std::fmt::Arguments::new_v1({
                                                                                                                                                                    static __STATIC_FMTSTR:
                                                                                                                                                                           &'static [&'static str]
                                                                                                                                                                           =
                                                                                                                                                                        &["",
                                                                                                                                                                          ", "];
                                                                                                                                                                    __STATIC_FMTSTR
                                                                                                                                                                },
                                                                                                                                                                &match (&rule,)
                                                                                                                                                                     {
                                                                                                                                                                     (__arg0,)
                                                                                                                                                                     =>
                                                                                                                                                                     [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                                                                                                  ::std::fmt::Debug::fmt)],
                                                                                                                                                                 })));
                            {
                                ::rt::begin_panic_fmt(&::std::fmt::Arguments::new_v1({
                                                                                         static __STATIC_FMTSTR:
                                                                                                &'static [&'static str]
                                                                                                =
                                                                                             &["no pattern matched in ",
                                                                                               "; failed at [",
                                                                                               "...]"];
                                                                                         __STATIC_FMTSTR
                                                                                     },
                                                                                     &match (&"_rule_name",
                                                                                             &next)
                                                                                          {
                                                                                          (__arg0,
                                                                                           __arg1)
                                                                                          =>
                                                                                          [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                                       ::std::fmt::Display::fmt),
                                                                                           ::std::fmt::ArgumentV1::new(__arg1,
                                                                                                                       ::std::fmt::Display::fmt)],
                                                                                      }),
                                                      {
                                                          static _FILE_LINE_COL:
                                                                 (&'static str,
                                                                  u32, u32) =
                                                              ("src/main.rs",
                                                               52u32, 0u32);
                                                          &_FILE_LINE_COL
                                                      })
                            }
                        }
                    }
                }
            }
        }
    }
}
impl <'input, T: ::Input<'input>> ::Parser<'input, T> for Rdp<T> {
    type
    Rule
    =
    Rule;
    type
    Token
    =
    Token<Rule>;
    #[inline]
    fn input(&self) -> &T { &self.input }
    #[inline]
    fn input_mut(&mut self) -> &mut T { &mut self.input }
    #[inline]
    fn end(&self) -> bool { self.input.len() == self.input.pos() }
    #[inline]
    fn eoi_matched(&self) -> bool { self.eoi_matched }
    #[inline]
    fn reset(&mut self) {
        self.input.set_pos(0);
        self.queue.clear();
        self.queue_index.set(0);
        self.failures.clear();
        self.fail_pos = 0;
    }
    #[inline]
    fn queue(&self) -> &Vec<Token<Rule>> { &self.queue }
    #[inline]
    fn queue_mut(&mut self) -> &mut Vec<Token<Rule>> { &mut self.queue }
    fn queue_with_captures(&self) -> Vec<(Token<Rule>, String)> {
        self.queue.clone().into_iter().map(|t|
                                               (t,
                                                self.input().slice(t.start,
                                                                   t.end).to_owned())).collect()
    }
    #[inline]
    fn queue_index(&self) -> usize { self.queue_index.get() }
    #[inline]
    fn inc_queue_index(&self) {
        self.queue_index.set(self.queue_index.get() + 1);
    }
    #[inline]
    fn set_queue_index(&self, index: usize) { self.queue_index.set(index); }
    #[inline]
    fn skip(&mut self) {
        if self.atomic { return }
        loop  { if !self.whitespace() { break  } }
        while self.comment() { loop  { if !self.whitespace() { break  } } }
    }
    #[inline]
    fn is_atomic(&self) -> bool { self.atomic }
    #[inline]
    fn set_atomic(&mut self, value: bool) { self.atomic = value; }
    #[inline]
    fn track(&mut self, failed: Rule, pos: usize) {
        if self.atomic { return }
        if self.failures.is_empty() {
            self.failures.push(failed);
            self.fail_pos = pos;
        } else {
            if pos == self.fail_pos {
                self.failures.push(failed);
            } else if pos > self.fail_pos {
                self.failures.clear();
                self.failures.push(failed);
                self.fail_pos = pos;
            }
        }
    }
    fn tracked_len_pos(&self) -> (usize, usize) {
        (self.failures.len(), self.fail_pos)
    }
    fn expected(&mut self) -> (Vec<Rule>, usize) {
        self.failures.sort();
        self.failures.dedup();
        (self.failures.iter().cloned().collect(), self.fail_pos)
    }
    #[inline]
    fn stack(&self) -> &Vec<String> { &self.stack }
    #[inline]
    fn stack_mut(&mut self) -> &mut Vec<String> { &mut self.stack }
}
const PLAIN_RULE: &str = "hello_world";
const QUOTED_RULE: &str = "\'hell\u{f8} w\u{f8}rld\'";
const SIMPLE_GRAMMAR: &str = "\n    rule: rule rule\n";
const TOML_SPEC: &str =
    r##"

key: $(KEY | STRING)
endl: COMMENT? NEWLINE
end: COMMENT? (EOF | NEWLINE)
scope: "[" $$key ( . $$key )* "]" end
array: "[" endl* "]"! $$expr "]"! ("," endl* "]"! $$expr endl* "]"!)%
inline_table: "{" endl* (($$entry endl* ","?) | ($$entry endl* ("," endl* $$entry endl*)* ","?))? endl* "}"
expr: $(INT | FLOAT | STRING | array | inline_table)
entry: $key "=" $expr

document: (($$entry | $$scope)? end)+

"##;
macro_rules! parse_and_print((
                             $ grammar : expr , $ grammar_rule : ident , $
                             reducer : ident ) => {
                             {
                             let mut parser = Rdp :: new (
                             StringInput :: new ( $ grammar ) ) ; parser . $
                             grammar_rule (  ) ; println ! ( "Queue:" ) ; for
                             token in parser . queue (  ) {
                             println ! ( "  {:?}" , token ) ; } println ! ( ""
                             ) ; if ! parser . end (  ) {
                             println ! (
                             "Parsing error: Expected rules: {:?}" , parser .
                             expected (  ) ) ; } else {
                             println ! (
                             "Reduced: {:#?}" , parser . $ reducer (  ) ) ; }
                             } });
fn main() {
    ::io::_print(::std::fmt::Arguments::new_v1({
                                                   static __STATIC_FMTSTR:
                                                          &'static [&'static str]
                                                          =
                                                       &["Hello, world!\n"];
                                                   __STATIC_FMTSTR
                                               }, &match () { () => [], }));
    {
        let mut parser = Rdp::new(StringInput::new(PLAIN_RULE));
        parser.rule_name();
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["Queue:\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        for token in parser.queue() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["  ", "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&token,) {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        }
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        if !parser.end() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["Parsing error: Expected rules: ",
                                                                 "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&parser.expected(),)
                                                            {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        } else {
            ::io::_print(::std::fmt::Arguments::new_v1_formatted({
                                                                     static __STATIC_FMTSTR:
                                                                            &'static [&'static str]
                                                                            =
                                                                         &["Reduced: ",
                                                                           "\n"];
                                                                     __STATIC_FMTSTR
                                                                 },
                                                                 &match (&parser._rule_name(),)
                                                                      {
                                                                      (__arg0,)
                                                                      =>
                                                                      [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                   ::std::fmt::Debug::fmt)],
                                                                  },
                                                                 {
                                                                     static __STATIC_FMTARGS:
                                                                            &'static [::std::fmt::rt::v1::Argument]
                                                                            =
                                                                         &[::std::fmt::rt::v1::Argument{position:
                                                                                                            ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                        format:
                                                                                                            ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                               ' ',
                                                                                                                                           align:
                                                                                                                                               ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                           flags:
                                                                                                                                               4u32,
                                                                                                                                           precision:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                           width:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,},}];
                                                                     __STATIC_FMTARGS
                                                                 }));
        }
    };
    {
        let mut parser = Rdp::new(StringInput::new(QUOTED_RULE));
        parser.rule_name();
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["Queue:\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        for token in parser.queue() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["  ", "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&token,) {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        }
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        if !parser.end() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["Parsing error: Expected rules: ",
                                                                 "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&parser.expected(),)
                                                            {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        } else {
            ::io::_print(::std::fmt::Arguments::new_v1_formatted({
                                                                     static __STATIC_FMTSTR:
                                                                            &'static [&'static str]
                                                                            =
                                                                         &["Reduced: ",
                                                                           "\n"];
                                                                     __STATIC_FMTSTR
                                                                 },
                                                                 &match (&parser._rule_name(),)
                                                                      {
                                                                      (__arg0,)
                                                                      =>
                                                                      [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                   ::std::fmt::Debug::fmt)],
                                                                  },
                                                                 {
                                                                     static __STATIC_FMTARGS:
                                                                            &'static [::std::fmt::rt::v1::Argument]
                                                                            =
                                                                         &[::std::fmt::rt::v1::Argument{position:
                                                                                                            ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                        format:
                                                                                                            ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                               ' ',
                                                                                                                                           align:
                                                                                                                                               ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                           flags:
                                                                                                                                               4u32,
                                                                                                                                           precision:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                           width:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,},}];
                                                                     __STATIC_FMTARGS
                                                                 }));
        }
    };
    {
        let mut parser = Rdp::new(StringInput::new(SIMPLE_GRAMMAR));
        parser.rules();
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["Queue:\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        for token in parser.queue() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["  ", "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&token,) {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        }
        ::io::_print(::std::fmt::Arguments::new_v1({
                                                       static __STATIC_FMTSTR:
                                                              &'static [&'static str]
                                                              =
                                                           &["\n"];
                                                       __STATIC_FMTSTR
                                                   },
                                                   &match () { () => [], }));
        if !parser.end() {
            ::io::_print(::std::fmt::Arguments::new_v1({
                                                           static __STATIC_FMTSTR:
                                                                  &'static [&'static str]
                                                                  =
                                                               &["Parsing error: Expected rules: ",
                                                                 "\n"];
                                                           __STATIC_FMTSTR
                                                       },
                                                       &match (&parser.expected(),)
                                                            {
                                                            (__arg0,) =>
                                                            [::std::fmt::ArgumentV1::new(__arg0,
                                                                                         ::std::fmt::Debug::fmt)],
                                                        }));
        } else {
            ::io::_print(::std::fmt::Arguments::new_v1_formatted({
                                                                     static __STATIC_FMTSTR:
                                                                            &'static [&'static str]
                                                                            =
                                                                         &["Reduced: ",
                                                                           "\n"];
                                                                     __STATIC_FMTSTR
                                                                 },
                                                                 &match (&parser.main(),)
                                                                      {
                                                                      (__arg0,)
                                                                      =>
                                                                      [::std::fmt::ArgumentV1::new(__arg0,
                                                                                                   ::std::fmt::Debug::fmt)],
                                                                  },
                                                                 {
                                                                     static __STATIC_FMTARGS:
                                                                            &'static [::std::fmt::rt::v1::Argument]
                                                                            =
                                                                         &[::std::fmt::rt::v1::Argument{position:
                                                                                                            ::std::fmt::rt::v1::Position::At(0usize),
                                                                                                        format:
                                                                                                            ::std::fmt::rt::v1::FormatSpec{fill:
                                                                                                                                               ' ',
                                                                                                                                           align:
                                                                                                                                               ::std::fmt::rt::v1::Alignment::Unknown,
                                                                                                                                           flags:
                                                                                                                                               4u32,
                                                                                                                                           precision:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,
                                                                                                                                           width:
                                                                                                                                               ::std::fmt::rt::v1::Count::Implied,},}];
                                                                     __STATIC_FMTARGS
                                                                 }));
        }
    };
}
