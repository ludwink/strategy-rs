// El trait ProcesadorDePago actúa como una interfaz común
// que define una familia de algoritmos o estrategias para procesar pagos.
trait ProcesadorDePago {
    fn procesar_pago(&self, monto: f64) -> Result<(), String>;
}

// TarjetaCredito es una de las "estrategias" concretas en el patrón Strategy.
struct TarjetaCredito {
    numero: String,
    nombre_titular: String,
    vencimiento: String,
}

// Otra estrategia concreta es PayPal
struct PayPal {
    cuenta_email: String,
}

// Implementación del trait ProcesadorDePago para la estrategia "TarjetaCredito".
impl ProcesadorDePago for TarjetaCredito {
    fn procesar_pago(&self, monto: f64) -> Result<(), String> {
        println!(
            "Procesando pago de ${} con tarjeta de crédito a nombre de {} (vencimiento: {})",
            monto, self.nombre_titular, self.vencimiento
        );
        // Simulación de validación de número
        if self.numero.is_empty() {
            Err("Número de tarjeta inválido.".to_string())
        } else if self.vencimiento != "12/24" {
            // Simulación de vencimiento
            Err("La tarjeta ha expirado.".to_string())
        } else {
            Ok(())
        }
    }
}

// Implementación del trait ProcesadorDePago para la estrategia "PayPal".
impl ProcesadorDePago for PayPal {
    fn procesar_pago(&self, monto: f64) -> Result<(), String> {
        println!(
            "Procesando pago de ${} a través de PayPal con la cuenta {}",
            monto, self.cuenta_email
        );
        // Simulación de la validación de PayPal
        if self.cuenta_email.is_empty() {
            Err("Cuenta de PayPal inválida.".to_string())
        } else {
            Ok(())
        }
    }
}

// Esta función es parte del **Contexto** en el patrón Strategy.
// El contexto (`procesar`) no sabe ni le importa cuál es la estrategia concreta.
// Solo sabe que el objeto pasado (de tipo &dyn ProcesadorDePago) sigue el "contrato" del trait.
fn procesar(procesador: &dyn ProcesadorDePago, monto: f64) {
    // Aquí se utiliza polimorfismo dinámico. Dependiendo del tipo de estrategia concreta
    // (TarjetaCredito, PayPal o CriptoMoneda) se ejecutará su implementación de procesar_pago.
    match procesador.procesar_pago(monto) {
        Ok(_) => println!("Pago procesado exitosamente."),
        Err(e) => println!("Hubo un error al procesar el pago: {}", e),
    }
}

fn main() {
    // Contexto en acción: Definimos las estrategias concretas que implementan el trait.
    let tarjeta = TarjetaCredito {
        numero: "1234-5678-9012-3456".to_string(),
        nombre_titular: "Juan Pérez".to_string(),
        vencimiento: "12/24".to_string(),
    };

    let paypal = PayPal {
        cuenta_email: "juan.perez@email.com".to_string(),
    };

    // En este punto, el contexto (`procesar`) no necesita saber qué tipo de pago está manejando.
    // Solo necesita que las "estrategias" implementen el comportamiento definido en el trait.
    procesar(&tarjeta, 100.0);
    procesar(&paypal, 200.0);
}
