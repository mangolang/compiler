package org.mangolang.util

// TODO: can this be done without less specifying of generic types?

/**
 * This 'enum' represents the result of an operation that may fail. It has two options.
 */
sealed class Result<Value, Error> {}

/**
 * This is the Result value for operations that succeeded.
 */
final class Success<Value, Error>(val value: Value): Result<Value, Error>() {}

/**
 * This is the result value for operations that failed.
 */
final class Failure<Value, Error>(val error: Error): Result<Value, Error>() {}


