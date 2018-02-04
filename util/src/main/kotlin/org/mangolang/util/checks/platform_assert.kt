package org.mangolang.util.checks

/**
 * @ImplNote This function is necessary because otherwise `assert` shadows the Java function of the same name.
 * @ImplNote And it must be in a separate file, because JVM and common both generated AssertKt class.
 */
expect inline internal fun platform_assert(condition: Boolean, message: Lazy<String>)

