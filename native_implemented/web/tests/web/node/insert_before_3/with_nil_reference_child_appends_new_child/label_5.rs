//! ```elixir
//! # label 5
//! # pushed to stack: (document, parent)
//! # returned form call: :ok
//! # full stack: (:ok, document, parent)
//! # returns: {:ok, new_child}
//! {:ok, new_child} = Lumen.Web.Document.create_element(document, "ul");
//! {:ok, inserted_child} = Lumen.Web.insert_before(parent, new_child, nil)
//! ```

use liblumen_alloc::erts::exception;
use liblumen_alloc::erts::process::Process;
use liblumen_alloc::erts::term::prelude::*;

use super::label_6;

#[native_implemented::label]
fn result(process: &Process, ok: Term, document: Term, parent: Term) -> exception::Result<Term> {
    assert_eq!(ok, Atom::str_to_term("ok"));
    assert!(document.is_boxed_resource_reference());
    assert!(parent.is_boxed_resource_reference());

    let new_child_tag = process.binary_from_str("ul")?;
    process.queue_frame_with_arguments(
        liblumen_web::document::create_element_2::frame()
            .with_arguments(false, &[document, new_child_tag]),
    );

    process.queue_frame_with_arguments(label_6::frame().with_arguments(true, &[parent]));

    Ok(Term::NONE)
}
