# voltaic 2 signature sidecar — generated, do not edit
enter : TemplateTable -> TemplateScope @ table shared
lookup : &str -> Option<voltaic_ast::DeriveTemplate> ! none @ name shared
drop : TemplateScope -> () @ self shared
from_items : &[Item] -> TemplateTable @ items shared
get : &str -> TemplateTable -> Option<DeriveTemplate> ! none @ name shared, self shared
TemplateScope.previous field
TemplateScope.enter method
TemplateScope.lookup method
TemplateScope.drop method
TemplateTable.by_name field
TemplateTable.from_items method
TemplateTable.get method
