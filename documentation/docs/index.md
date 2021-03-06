# The Sy scripting language

A small, incredibly useless language.

[![github](https://img.shields.io/badge/Get%20it%20on-GitHub-orange)](https://github.com/SLaGrave/sy-script)

```
   _____                     _       _   
  / ____|                   (_)     | |  
 | (___  _   _ ___  ___ _ __ _ _ __ | |_ 
  \___ \| | | / __|/ __| '__| | '_ \| __|
  ____) | |_| \__ \ (__| |  | | |_) | |_ 
 |_____/ \__, |___/\___|_|  |_| .__/ \__|
          __/ |               | |        
         |___/                |_|        
```

## How

Write a script and call it with syscript.py. A tutorial on the whole "write a script" step can be found [here](write.md).

```sh
$ python syscript ./hello.sy
```

The SyParser python object can also be loaded into other project to allow usage of Sy code in python projects. Instructions coming soon (or you can peek around in syscript.py to see how that implements it).

## Why

- I'm interested in writing programming languages.

- A coworker and I have frequently joked about using an abnormal or esoteric language for a project, so this is my attempt at writing a language to fit that description.

- I liked the wiki article for [OISCs](https://en.wikipedia.org/wiki/One-instruction_set_computer).