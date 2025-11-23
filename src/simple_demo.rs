//! A simple Bevy demo with a floor, sphere, and light.

use bevy::prelude::*;

pub struct SimpleDemoPlugin;

impl Plugin for SimpleDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene);
        app.add_systems(Update, animate_sphere);
    }
}

const SPHERE_RADIUS: f32 = 0.5;
const GRAVITY: f32 = -9.8;
const BOUNCE_DAMPING: f32 = 0.8; // Energy loss on bounce (0.8 = 80% of energy retained)
const INITIAL_HEIGHT: f32 = 2.0; // Starting height above floor
const REST_DURATION: f32 = 2.0; // Seconds to wait at rest before bouncing again
const BOUNCE_UP_VELOCITY: f32 = 8.0; // Initial upward velocity when bouncing from rest

/// Marker component to identify the animated sphere
#[derive(Component)]
struct AnimatedSphere {
    velocity: f32, // Vertical velocity
    rest_timer: f32, // Time spent at rest (zero velocity)
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a floor (plane)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
    ));

    // Add a sphere with animation marker and initial velocity
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(SPHERE_RADIUS).mesh().uv(32, 18))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.9))),
        Transform::from_xyz(0.0, INITIAL_HEIGHT, 0.0),
        AnimatedSphere {
            velocity: 0.0, // Start at rest
            rest_timer: 0.0, // Timer for tracking rest duration
        },
    ));

    // Add a light
    commands.spawn((
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    // Add a camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 3.0, 5.0).looking_at(Vec3::new(0.0, SPHERE_RADIUS + 1.0, 0.0), Vec3::Y),
    ));
}

/// Animates the sphere with realistic physics-based bouncing
fn animate_sphere(time: Res<Time>, mut query: Query<(&mut Transform, &mut AnimatedSphere)>) {
    let dt = time.delta_secs();
    let floor_y = SPHERE_RADIUS;
    
    for (mut transform, mut sphere) in query.iter_mut() {
        // Check if sphere is at rest (on floor with zero velocity)
        let is_at_rest = sphere.velocity.abs() < 0.1 && transform.translation.y <= floor_y + 0.01;
        
        if is_at_rest {
            // Increment rest timer
            sphere.rest_timer += dt;
            
            // If we've been at rest long enough, bounce back up
            if sphere.rest_timer >= REST_DURATION {
                sphere.velocity = BOUNCE_UP_VELOCITY;
                sphere.rest_timer = 0.0; // Reset timer
            }
        } else {
            // Reset timer when moving
            sphere.rest_timer = 0.0;
            
            // Apply gravity to velocity
            sphere.velocity += GRAVITY * dt;
            
            // Update position based on velocity
            transform.translation.y += sphere.velocity * dt;
            
            // Bounce off the floor (floor is at y = 0, sphere center should be at SPHERE_RADIUS)
            if transform.translation.y <= floor_y {
                transform.translation.y = floor_y;
                // Reverse velocity and apply damping (energy loss)
                sphere.velocity = -sphere.velocity * BOUNCE_DAMPING;
                
                // Stop bouncing if velocity is too small (prevent infinite tiny bounces)
                if sphere.velocity.abs() < 0.1 {
                    sphere.velocity = 0.0;
                    transform.translation.y = floor_y;
                }
            }
        }
    }
}
