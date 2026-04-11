# actrpc-interceptor

`actrpc-interceptor` provides the Rust-side API for implementing ActRPC interceptors.

An interceptor receives an `InterceptionRequest` and returns an `InterceptionResponse`.

## Purpose

This crate defines the runtime-side structure for interceptor implementations.

It exists to make interceptor endpoints easy to implement while keeping protocol definitions in `actrpc-core`.

## What It Contains

- interceptor-side traits
- interceptor runtime helpers
- response construction helpers
- optional utilities for serving interceptor endpoints

## Design

- Built on `actrpc-core`
- Focused on interceptor implementation
- Independent from orchestrator internals
- Compatible with different transport layers

## Interceptor Model

An interceptor:

- receives an `InterceptionRequest`
- inspects the message and prior actions
- decides whether to request actions
- decides whether processing should continue

It does not execute actions itself.

## Scope

This crate does not include:

- protocol definitions
- orchestrator logic
- built-in interceptors
- built-in actions

## Summary

`actrpc-interceptor` is the implementation crate for ActRPC interceptors.

It defines how interceptors are written, not how the overall pipeline is run.
