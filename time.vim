" Vim syntax file
" Copy this file to ~/.config/nvim/syntax/time.vim
" Language: Timesheet
" Maintainer: Alexei Miatlitski
" Latest Revision: 17 Aug 2021

if exists("b:current_syntax")
  finish
endif

syn match timeDate     /\v^(\d{2}\.\d{2}\.\d{4}|\d{4}-\d{2}-\d{2})$/
syn match timeLabel    /\v([a-zA-Z]{2,5}-\d{1,6}|--)/
syn match timeNotation /\v^((\s*(\d{1,6}(h|m)))+)/
syn match timeComment  /\v^#.+$/

let b:current_syntax = "time"

hi def link timeDate       Typedef
hi def link timeLabel      Special
hi def link timeNotation   Number
hi def link timeComment    Comment
