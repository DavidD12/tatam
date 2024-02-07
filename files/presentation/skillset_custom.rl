
type Position
type T

skillset custom_robot {

    resource {

        battery {
            state { Normal Low Critical }
            initial Normal
            transition {
                Normal -> Low
                Normal -> Critical
                Low -> Normal
                Low -> Critical
                Critical -> Normal
            }
        }

        motion {
            state { On Off }
            initial Off
            transition {
                On -> Off
                Off -> On
            }
        }

    }

    event {
        from_Normal_to_Low {
            guard battery == Normal
            effect battery -> Low
        }
    }


    skill goto {

        input {
            target: Position
        }

        output result: T

        precondition {
            battery_not_critical : battery == Normal
            can_move : motion == Off
        }

        start motion -> On

        invariant {
            in_movement {
                guard motion == On
            }
            battery_not_critical {
                guard battery != Critical
                effect motion -> Off
            }
        }

        interrupt {
            interrupting true
            effect {
                motion -> Off
                battery -> Low
            }
        }

        success {
            arrived {
                effect {
                    motion -> Off
                    battery -> Low
                }
            }
            arrived_2 {
                effect {
                    motion -> Off
                    battery -> Critical
                }
            }
        }

        failure {
            blocked {
                effect {
                    motion -> Off
                    battery -> Low
                }
            }
        }
        
    }

    skill retoho {

        output result: T

        precondition {
            can_move : motion == Off
        }

        start motion -> On

        invariant {
            in_movement {
                guard motion == On
            }
        }

        interrupt {
            interrupting true
            effect motion -> Off
        }

        success arrived {
            effect motion -> Off
        }

        failure {
            blocked {
                effect motion -> Off
            }
        }
        
    }

    skill recharge {

        output result: T

        precondition {
            battery_normal : battery != Normal
            dont_move : motion == Off
        }

        invariant {
            not_in_movement {
                guard motion == Off
            }
        }

        success charged {
            effect battery -> Normal
        }

        failure {
            emergency {}
        }
        
    }
}