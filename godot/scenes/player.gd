extends CharacterBody3D

# Movement settings
@export var speed := 14.0
@export var fall_acceleration := 75.0

# Look settings
@export var mouse_sensitivity := 0.1
@export var invert_y := false
@export var clamp_vertical_rotation := true
@export var vertical_rotation_limit := 90.0  # Degrees

# Nodes
@onready var pivot := $Pivot2  # Node for horizontal rotation (yaw)
@onready var camera := $Pivot2/Camera3D  # Camera for vertical rotation (pitch)

var target_velocity := Vector3.ZERO

func _ready():
	# Hide and capture the mouse cursor
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _input(event: InputEvent):
	if event is InputEventMouseMotion:
		# Rotate the player horizontally (yaw)
		pivot.rotate_y(deg_to_rad(-event.relative.x * mouse_sensitivity))

		# Rotate the camera vertically (pitch)
		var camera_rotation = -event.relative.y * mouse_sensitivity  # Inverted for correct look direction
		if invert_y:
			camera_rotation = -camera_rotation
		camera.rotate_x(deg_to_rad(camera_rotation))

		# Clamp vertical rotation to avoid looking too far up or down
		if clamp_vertical_rotation:
			camera.rotation_degrees.x = clamp(
				camera.rotation_degrees.x,
				-vertical_rotation_limit,
				vertical_rotation_limit
			)

func _physics_process(delta):
	var direction = Vector3.ZERO

	if Input.is_action_pressed("move_right"):
		direction.x += 1
	if Input.is_action_pressed("move_left"):
		direction.x -= 1
	if Input.is_action_pressed("move_backward"):
		direction.z += 1
	if Input.is_action_pressed("move_forward"):
		direction.z -= 1

	if direction != Vector3.ZERO:
		direction = direction.normalized()
		# Transform the direction from local space (camera) to global space
		direction = pivot.basis * direction

	# Ground Velocity
	target_velocity.x = direction.x * speed
	target_velocity.z = direction.z * speed

	# Vertical Velocity
	if not is_on_floor():  # If in the air, fall towards the floor. Literally gravity
		target_velocity.y -= fall_acceleration * delta

	# Moving the Character
	velocity = target_velocity
	move_and_slide()

	# Lock the camera to the player's position
	camera.global_transform.origin = pivot.global_transform.origin
