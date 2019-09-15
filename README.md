# image-energy-thing

This project is a simple image filtering algorithm that does edge finding to find an "energy" value for every pixel

In order, arguments are

image path - a path to your image

* no default value

exponent - any floating point, the final energy values will be calculated as (energy)^(1/exponent), in general, the larger this value is, the more saturated the output will be

* default value is 3.0
  
function - "alpha", "fillavg" or "mono"

* alpha currently does nothing and still needs to be implemented, but the goal is to "show" the image underneath based on the energy
  
* fill average currently does nothing and still needs to be implemented, but the goal is to essentially flood fill the energy, calculate the average color of that region, and fill the region with that color for the output
  
* mono will simply output the calculated energy
  
* default value is mono

mode - "component" or "combined"

* component will take the invididual energy values of each channel in the image

* combined will take the energy of an entire pixel

* default is combined

color space - "rgb" or "lab"

* rgb will sample in sRGB color space

* lab will sample in CIELAB color space, this is usually the recommended for combined energy

* default is lab

convert component lab to rgb - "y" or "n"

* if component mode and lab color space are used, this will give you the option to properly convert the component lab values back to rgb I found the outputs interesting for both so I decided to have this option

* default is n
