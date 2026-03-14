{ pkgs, crane, toolchain }:

(crane.mkLib pkgs).overrideToolchain toolchain