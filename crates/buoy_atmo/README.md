# Buoy Atmosphere

Buoy's atmosphere model is an implementation of the 
[*U.S. Standard Atmosphere, 1976*](./us-standard-atmosphere_st76-1562_noaa.pdf).
This model approximates Earth atmosphere temperatures and pressures for
altitudes between -57 to 85,000 kilometers above sea level. It does not model
humidity or wind currents, and is perfectly uniform in the X-Z plane (each slice
parallel to the ground plane is uniform along that plane).

This crate provides the the U.S. Standard Atmosphere 1976 as a Bevy resource,
though it is functionally just a lookup table with engineering units.

## notes

The crate is set up in a way that is supposed to allow other atmosphere models
to be used in place of the default one by specifying corresponding features. So
for example if a user wanted to use *US Standard Atmosphere 1976* they'd use
`bevy_atmo = {features = ["us_std_1976"]}` and to use *A Different Model* they'd
use `bevy_atmo = {features = ["different"]}`.