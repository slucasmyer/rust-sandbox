fn nearest_neighbor(locations: &[Location]) -> Vec<Location> {
    let mut route = Vec::new();
    let mut unvisited = locations.to_vec();

    // Start at the first location
    let mut current = unvisited.remove(0);
    route.push(current);

    while !unvisited.is_empty() {
        // Find the nearest unvisited location
        let mut nearest = None;
        let mut nearest_distance = std::f64::INFINITY;
        for (i, location) in unvisited.iter().enumerate() {
            let distance = current.distance(location);
            if distance < nearest_distance {
                nearest = Some(i);
                nearest_distance = distance;
            }
        }

        // Add the nearest location to the route and make it the current location
        let next = unvisited.remove(nearest.unwrap());
        route.push(next);
        current = next;
    }

    // Return to the starting point
    route.push(route[0]);

    route
}
