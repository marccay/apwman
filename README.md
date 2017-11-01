# apwman

Yet Another GPG Password Manager
	GPG depenency
	user chooses file managment

Basic Syntax:
		
		apwman [command] FILE
	

Commands:

	"h" | "help" 	:: print help/info prompt
	"r" | "read" 	:: decrypt file and print file content
	"n" | "new" 	:: creates and encrypt password file, prompts for info
			


Decrypt Example:

	$ apwman read example

Output:

	[decrypted]: example

	test@example.com
	password123


Encrypt Example:

	$ apwman new example

Output:
	Username:
		<prompt>
	Password:
		<prompt>


