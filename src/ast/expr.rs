use paste::paste;

macro_rules! expression_nodes_impl {
    ( $(
        $(#[doc = $node_doc:expr])*
        $name:ident { $( $(#[doc = $param_doc:expr])* $param:ident : $type:ty, )* },
    )* ) => {
        paste! {
            // pub enum ExpressionNode {
            //     $(
            //         $(#[doc = $node_doc])*
            //         $name {
            //             $( $(#[doc = $param_doc])* $param : $type ,)*
            //         },
            //     )*
            // }

            pub trait ExpressionNodeVisitor {
                type Output;

                $(
                    fn [<visit_ $name:snake>](&mut self) -> Self::Output;
                )*

                fn visit(&mut self) -> Self::Output {
                    // self.visit_query()
                    todo!()
                }
            }
        }
    };

}

expression_nodes_impl! {

    /// Expression: OrExpression
    Expression {},

    /// OrExpression: Vec<XorExpression>
    OrExpression {},

    /// XorExpression: Vec<AndExpression>
    XorExpression {},

    /// AndExpression: Vec<NotExpression>
    AndExpression {},

    /// NotExpression: Not? ComparsionExpression
    NotExpression {},

    /// ComparsionExpression: AddOrSubtractExpression + Vec<PartialComparisonExpression>
    ComparisonExpression {},

    /// AddOrSubtractExpression: MultiplyDivideModuloExpression (+/- MultiplyDivideModuloExpression)*
    AddOrSubtractExpression {},

    /// MultiplyDivideModuloExpression: PowerOfExpression (*///% PowerOfExpression)*
    MultiplyDivideModuloExpression {},

    /// PowerOfExpression: UnaryAddOrSubtractExpression (^ UnaryAddOrSubtractExpression)*
    PowerOfExpression {},

    /// UnaryAddOrSubtractExpression: (+/-)* StringListNullOperatorExpression
    UnaryAddOrSubtractExpression {},

    /// StringListNullOperatorExpression: PropertyOrLabelsExpression, (StringOperatorExpression|ListOperatorExpression|NullOperatorExpression)*
    StringListNullOperatorExpression {},

    /// PropertyOrLabelsExpression: Atom, (PropertyLookup)*, (NodeLabels)+
    PropertyOrLabelsExpression {},

    /// Atom: IS NOT NULL
    Atom {},
}
