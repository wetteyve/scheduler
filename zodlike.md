### Minimal Zod-like Schema Validation in Rust **with napi-rs bindings**

> You are a senior Rust engineer with deep experience in **napi-rs** and Node.js interop.
>
> I am building a **Zod-like schema validation system in Rust**, exposed to **Node.js via napi-rs**.
>
> This is **version 1** and must remain **intentionally minimal**.
>
> ---
>
> ## Supported schema types (v1 ONLY)
>
> * `boolean`
> * `number` (`f64`)
> * `string`
> * `array`
>
> ❌ Do NOT implement objects, unions, refinements, optionals, coercions, or transformations.
>
> ---
>
> ## High-level design requirements
>
> ### 1. Two-layer architecture (IMPORTANT)
>
> The implementation must be **explicitly split into two conceptual layers**:
>
> ```
> [ Core validation engine (pure Rust) ]
>                |
>                v
> [ napi-rs adapter / JS bindings ]
> ```
>
> * The **core layer** must:
    >
    >   * Be completely independent of napi-rs
>   * Contain all validation logic
>   * Use Rust enums and structs
> * The **napi-rs layer** must:
    >
    >   * Convert JS values into core `Value`
>   * Convert core results/errors back into JS-friendly shapes
>   * Expose the public API to Node.js
>
> Do NOT mix validation logic with N-API code.
>
> ---
>
> ## Core validation engine (Rust-only)
>
> ### Input value representation
>
> ```rust
> enum Value {
>     Bool(bool),
>     Number(f64),
>     String(String),
>     Array(Vec<Value>),
> }
> ```
>
> ### Schema definition
>
> Schemas should be constructible like:
>
> ```rust
> Schema::boolean()
> Schema::number()
> Schema::string()
> Schema::array(Schema)
> ```
>
> ### Parsing API (Zod-inspired)
>
> ```rust
> fn parse(&self, input: Value) -> Result<Value, ValidationError>
> fn safe_parse(&self, input: Value) -> SafeParseResult
> ```
>
> * `parse`:
    >
    >   * Stops on first error
>   * Returns a single `ValidationError`
> * `safe_parse`:
    >
    >   * Never panics
>   * Accumulates **all validation errors**
>   * Returns:
>
> ```rust
> enum SafeParseResult {
>     Success(Value),
>     Failure(Vec<ValidationError>),
> }
> ```
>
> ### Validation errors
>
> Errors must be **precise, structured, and nestable**.
>
> ```rust
> struct ValidationError {
>     path: Vec<PathSegment>,
>     expected: String,
>     received: String,
> }
>
> enum PathSegment {
>     Index(usize),
> }
> ```
>
> Example error:
>
> ```text
> expected number at path [2], received string
> ```
>
> ---
>
> ## Array behavior
>
> * Validate each element
> * Track index paths correctly
> * `parse` stops at first failing element
> * `safe_parse` collects **all element errors**
>
> ---
>
> ## napi-rs adapter layer
>
> ### JS ↔ Rust value conversion
>
> * Accept `JsUnknown` as input
> * Convert into core `Value`
> * Fail with a meaningful JS error if conversion is impossible
>
> ### JS-exposed API
>
> Expose a JS-friendly API equivalent to Zod:
>
> ```ts
> schema.parse(value)       // throws JS Error
> schema.safeParse(value)   // returns { success: true, data } | { success: false, errors }
> ```
>
> Where:
>
> * `parse` throws a `napi::Error`
> * `safeParse` returns a plain JS object
>
> Error objects returned to JS must include:
>
> ```ts
> {
>   path: (number | string)[],
>   expected: string,
>   received: string
> }
> ```
>
> ---
>
> ## Implementation constraints
>
> * Use **idiomatic Rust**
> * Avoid macros unless required by napi-rs
> * Prefer owned data (no borrowed JS references)
> * Avoid unnecessary lifetimes
> * Ensure all exported types are safe across the N-API boundary
>
> ---
>
> ## Output expectations
>
> * Provide **complete Rust code** (single module or small set of modules)
> * Include:
    >
    >   * Core validation engine
>   * napi-rs bindings
> * Comment on:
    >
    >   * Why the core/adapter separation matters
>   * How error paths are preserved across JS ↔ Rust
>
> ---
>
> ## Explicit non-goals (DO NOT IMPLEMENT)
>
> * Objects / records
> * Optional values
> * Refinements (`min`, `max`, etc.)
> * Async validation
> * Serialization formats
>
> ---
>
> ## Mental model
>
> This is **not about cloning Zod’s API**.
>
> This is about:
>
> * Designing a robust Rust core
> * Creating a clean napi-rs boundary
> * Establishing a foundation that can grow incrementally
>
> Start with the **simplest possible implementation** that satisfies all constraints.
