## How to create new component

### Steps:

1. **Create component.rs**: In src/components create the new component file "component.rs"

2. **Implement Functions in component.rs**: In the new file implement the following functions:
   - new(),
   - connect(&mut self, connection: &Connection),
   - get_connection(&self, connection_type: ConnectionType),
   - get_id(&self),

3. **Implement Functions in components.rs**: In the existing file src/components.rs extend the functions:
   - get_id,
   - get_name,
   - connect,
   - get_connection,
   - num_currents

4. **If the Component has a new Connection Type**: Implement the new connection type in src/components.rs

5. **Implement Solver Functions**: In the existing file src/solver extend the function solve with the new component
