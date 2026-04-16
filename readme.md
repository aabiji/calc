
Parse mathematical expression into a tree using pratt parsing
- Implement differentiation recursively, as well as tree simplifying techniques
- Implement various integration techniques
- Actually plot the functions in 2D using marching squares algorithm
    - https://www.desmos.com/calculator/a5fitp6hjt?lang=pt-BR
    - https://www.desmos.com/calculator/6ut1a9ljy7
- Support zoom and pan
- Graph things in 3D https://www.desmos.com/3d/c7642334f4
- Slope fields for differential equations

Parsing:
- Pratt parsing expressions to handle order of operations
- Parsing identifier + open paren as a function call, then making the distinction between
  function and multiplication during eval
- Implied parentheses and multiplications are handled using another tokenization pass
- Identifiers longer than 1 character are only allowed with subscripts.
  Otherwise, they are to be treated as several variables implicitly multipled together.
