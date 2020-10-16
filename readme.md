# Beancount


## todo list
 - [x] open directive
 - [x] close directive
 - [x] note directive
 - [x] commodity directive
 - [x] transaction directive
    - [x] optional single price
    - [x] optional total price
    - [x] optional actual cost
    - [x] tags
    - [x] links
 - [ ] push tag & pop tag
 - [x] balance directive
 - [x] pad directive
 - [x] document directive
 - [x] price directive
 - [x] event directive
 - [x] option directive
 - [x] plugin directive
 - [x] include directive
 - [x] custom directive
    - [ ] for now not support using number in custom like `1970-01-01 custom "budget" Expenses:Eat 100 CNY`, you must use like this`1970-01-01 custom "budget" Expenses:Eat "100" CNY` 
 - [x] comment supported
    - only support new line comment with `;` E.G. `; here is the comment.`