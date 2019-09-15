# image-energy-thing

This project is a simple image filtering algorithm that does edge finding to find an "energy" value for every pixel

In order, arguments are

image path - a path to your image

exponent - any floating point, the final energy values will be calculated as (energy)^(1/exponent), in general, the larger this value is, the more saturated the output will be
  default value is 3.0
  
function - "alpha", "fillavg" or "mono"
  alpha currently does nothing and still needs to be implemented  
  fill average currently does nothing and still needs to be implemented
  mono will simply output the calculated energy
  default value is mono

