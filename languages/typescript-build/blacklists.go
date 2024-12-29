package main

// isBlacklistedType checks if a type is blacklisted
func isBlacklistedType(typeName string) bool {
	switch typeName {
	case "Query", "Mutation", "Subscription", "__Schema", "__Type", "__TypeKind", "__Field", "__InputValue", "__EnumValue", "__Directive", "__DirectiveLocation":
		return true
	}

	return false
}
