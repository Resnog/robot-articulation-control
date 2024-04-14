// For RAMPS 1.4
#define X_STEP_PIN         54
#define X_DIR_PIN          55
#define X_ENABLE_PIN       38

#define Y_STEP_PIN         60
#define Y_DIR_PIN          61
#define Y_ENABLE_PIN       56

#define Z_STEP_PIN         46
#define Z_DIR_PIN          48
#define Z_ENABLE_PIN       62

#define E0_STEP_PIN        26
#define E0_DIR_PIN         28
#define E0_ENABLE_PIN      24

#define E1_STEP_PIN        36
#define E1_DIR_PIN         34
#define E1_ENABLE_PIN      30

void stepper_config(uint8_t step_pin,
                    uint8_t dir_pin,
                    uint8_t enable_pin){
    pinMode(step_pin, OUTPUT);
    pinMode(dir_pin, OUTPUT);
    pinMode(enable_pin, OUTPUT);
}

void stepper_start(uint8_t enable_pin){
    digitalWrite(enable_pin, HIGH);
}

void setup() {
  // put your setup code here, to run once:
  stepper_config(X_STEP_PIN, X_DIR_PIN,X_ENABLE_PIN);
  stepper_start(X_ENABLE_PIN);
}

void loop() {
  // put your main code here, to run repeatedly:
  delay(500);
  digitalWrite(X_STEP_PIN, LOW);
  delay(500);
  digitalWrite(X_STEP_PIN, HIGH);
}
