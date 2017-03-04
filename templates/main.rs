use format::field::*;
use nom::{be_u8,be_u16,be_u32,be_u64};

pub const DESCRIPTION: &'static str = "{{name}} - {{major_version}}.{{minor_version}}.{{revision}}";

pub enum Class {
  {{#each specs.classes as |class| ~}}
    {{camel class.name}}({{camel class.name}}Methods),
  {{/each ~}}
  None,
}

macro_rules! call_path (
  ($i: expr, $p: path) => ($p($i))
);

named!(pub parse_class<Class>,
  switch!(be_u16,
  {{#each specs.classes as |class| ~}}
    {{class.id}} => call!(parse_class_{{snake class.name}}) |
  {{/each ~}}
  _ => value!(Class::None)
  )
);

{{#each specs.classes as |class|}}
  pub enum {{camel class.name}}Methods {
    {{#each class.methods as |method| ~}}
      {{camel method.name}}({{camel class.name}}{{camel method.name}}),
    {{/each ~}}
    None,
  }

  named!(pub parse_class_{{snake class.name}}<Class>,
    switch!(be_u16,
      {{#each class.methods as |method| ~}}
      {{method.id}} => map!(call!(parse_class_{{snake class.name}}_method_{{snake method.name}}), |m| Class::{{camel class.name}}(m)) |
      {{/each ~}}
      _  =>  value!(Class::{{camel class.name}}({{camel class.name}}Methods::None))
    )
  );

  {{#each class.methods as |method|}}
    pub struct {{camel class.name}}{{camel method.name}} {
      {{#each method.arguments as |argument| ~}}
        pub {{snake argument.name}}: {{map_type argument}},
      {{/each ~}}

    }

    named!(parse_class_{{snake class.name}}_method_{{snake method.name}}<{{camel class.name}}Methods>,
      do_parse!(
        {{#each method.arguments as |argument| ~}}
          {{snake argument.name}}: {{map_parser argument}} >>
        {{/each ~}}

        ({{camel class.name}}Methods::{{camel method.name}}({{camel class.name}}{{camel method.name}} {
          {{#each method.arguments as |argument| ~}}
            {{snake argument.name}}: {{snake argument.name}},
          {{/each ~}}
        }))
      )
    );
  {{/each}}
{{/each}}
