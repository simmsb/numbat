# Unit Declarations

New units of measurement can be introduced with the `unit` keyword. There are two types of units: base units and derived units.

A new **base unit** can be defined by specifying the *physical dimension* it represents. For example, in the
 [International System of Units (SI)](https://en.wikipedia.org/wiki/International_System_of_Units), the *second* is the base unit for measuring times:
```
unit second: Time
```
Here, `Time` denotes the physical dimension. To learn more, you can read the [corresponding chapter](physical-dimensions.md). But for now, we can just assume that they are already given.

**Derived units** are also introduced with the `unit` keyword. But unlike base units, they are defined through their relation to
other units. For example, a *minute* can be defined as
```
unit minute: Time = 60 second
```
Here, the `: Time` annotation is optional. If a dimension is specified, it will be used to verify that the right hand side expression (`60 second`) is indeed of physical dimension `Time`. This is apparent in this simple example, but can be useful for more complicated unit definitions like
```
unit farad: Capacitance = ampere^2 second^4 / (kilogram meter^2)
```


## Prefixes

If a unit may be used with metric prefixes such as `milli`/`m`, `kilo`/`k` or `mega`/`M`, we can prepend the unit definition with the `@metric_prefixes` decorator:
```
@metric_prefixes
unit second: Time
```
This allows identifiers suchs as `millisecond` to be used in calculations. See the section below how prefixes interact with aliases.

Similarly, if a unit should be prependable with *binary* (IEC) prefixes such as `kibi`/`Ki`, `mebi`/`Mi` or `gibi`/`Gi`, you can
add the `@binary_prefixes` decorator. A unit might also allow for both metric and binary prefixes, for example:
```
@binary_prefixes
@metric_prefixes
unit byte = 8 bit
```
This allows the usage of both `mebibyte` (1024² byte) as well as `megabyte` (1000² byte).

## Aliases

It is often useful to define alternative names for a unit. For example, we might want to use the plural form `seconds` or the commonly
used short version `s`. We can use the `@aliases` decorator to specify "long" aliases and the `@aliases_short` to define "short" ones:
```
@metric_prefixes
@aliases(meters, metre, metres)
@aliases_short(m)
unit meter: Length
```
The reason we distinguish between long and short forms is that they interact differently with prefixes. The unit name itself and all
long aliases can be prefixed with the long versions of prefixes (..., `milli`, `kilo`, `mega`, `giga`, ...). On the other hand, short aliases can only be prefixed with the respective short versions of the prefixes (..., `m`, `k`, `M`, `G`, ...). The definition above
allows all of following expressions:
```
millimeter
kilometer

millimeters
kilometers

millimetre
kilometre

millimetres
kilometres

mm
km
...
```