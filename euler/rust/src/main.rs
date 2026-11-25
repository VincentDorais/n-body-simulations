// N-Body gravity simulation.
// Euler integrator.
// Single thread. Auto-optimizations.

// Meters, Kilograms, seconds.

use std::time::Instant;

const G: f64 = 6.6743e-11;
const N: usize = 27;

struct SBodies {
    x: [f64; N],
    y: [f64; N],
    z: [f64; N],
    vx: [f64; N],
    vy: [f64; N],
    vz: [f64; N],
    mass: [f64; N],
    radius: [f64; N],
}

fn main() {
    let dt: f64 = 1.0;
    let step: usize = 10_000_000;

    // Positions and velocities from NASA Horizons data for 26-09-2026 at 00:00:00.0000 TDB. Solar System Barycenter.
    // Masses and radii from Wikipedia.
    let mut solsys = SBodies {
        x: [
            -1.908487414960836_e8,  // Sun
            -1.991359984355149_e10, // Mercury
            1.0499520185477800_e11, // Venus
            1.4985628126514220_e11, // Earth
            1.5023609453631250_e11, // Moon
            2.8860581412857820_e10, // Mars
            -5.180202934546471_e11, // Jupiter
            -5.189142422390277_e11, // Ganymede
            -5.186876952078271_e11, // Callisto
            -5.183728483681735_e11, // Io
            -5.179648053059427_e11, // Europa
            1.3859665737388790_e12, // Saturn
            1.3859232009978700_e12, // Titan
            1.3864903674899490_e12, // Rhea
            1.3845642172171870_e12, // Iapetus
            1.3856145802590930_e12, // Dione
            1.3858614077392210_e12, // Tethys
            1.3861512250178400_e12, // Enceladus
            1.3384663788023680_e12, // Uranus
            1.3387625976246830_e12, // Titania
            1.3384329634074950_e12, // Oberon
            1.3382712547875050_e12, // Umbriel
            1.3384388872278930_e12, // Ariel
            4.4635734869182730_e12, // Neptune
            4.4637402686055140_e12, // Triton
            2.9844087233868480_e12, // Pluto
            2.9844211608676770_e12, // Charon
        ],
        y: [
            -7.708483695236095_e8,  // Sun
            -6.760385185792711_e10, // Mercury
            -2.788385821145211_e10, // Venus
            6.7058513945656970_e9,  // Earth
            6.6647035007779940_e9,  // Moon
            2.2955055657416670_e11, // Mars
            6.0062292119704140_e11, // Jupiter
            6.0121608320380090_e11, // Ganymede
            6.0238550013893700_e11, // Callisto
            6.0085603570881390_e11, // Io
            6.0129461316842930_e11, // Europa
            2.5943481015740690_e11, // Saturn
            2.5835715346153020_e11, // Titan
            2.5938564929816200_e11, // Rhea
            2.5625583252272930_e11, // Iapetus
            2.5933012346008860_e11, // Dione
            2.5968136674069510_e11, // Tethys
            2.5955851762713360_e11, // Enceladus
            2.5813516616229240_e12, // Uranus
            2.5813315085676570_e12, // Titania
            2.5814407655852080_e12, // Oberon
            2.5813691458371830_e12, // Umbriel
            2.5813316184433400_e12, // Ariel
            2.0333943854360820_e11, // Neptune
            2.0318924629098220_e11, // Triton
            -4.395251268785082_e12, // Pluto
            -4.395246760028754_e12, // Charon
        ],
        z: [
            1.4257422826508060_e7,  // Sun
            -3.638708605360989_e9,  // Mercury
            -6.427335092098974_e9,  // Venus
            -1.490024972805753_e6,  // Earth
            1.2975563339395450_e7,  // Moon
            4.1286053301584870_e9,  // Mars
            9.1017943978388010_e9,  // Jupiter
            9.1113992933885750_e9,  // Ganymede
            9.1480161320003570_e9,  // Callisto
            9.1051800271157030_e9,  // Io
            9.1231908979673980_e9,  // Europa
            -5.969501488151904_e10, // Saturn
            -5.913502531847332_e10, // Titan
            -5.971803461256965_e10, // Rhea
            -5.868229575249475_e10, // Iapetus
            -5.960622956277069_e10, // Dione
            -5.981740379289320_e10, // Tethys
            -5.977769123482867_e10, // Enceladus
            -7.753248151706457_e9,  // Uranus
            -7.432641617763758_e9,  // Titania
            -7.177519974507570_e9,  // Oberon
            -7.933368431657553_e9,  // Umbriel
            -7.941061654786944_e9,  // Ariel
            -1.070549795671838_e11, // Neptune
            -1.073297692001680_e11, // Triton
            -3.929487081633263_e11, // Pluto
            -3.929631619999628_e11, // Charon
        ],
        vx: [
            1.0522744354555900_e1, // Sun
            3.6949711575351460_e4, // Mercury
            8.5726349624921720_e3, // Venus
            -1.815389568008479_e3, // Earth
            -1.756791179725980_e3, // Moon
            -2.311029638337283_e4, // Mars
            -1.004921344065230_e4, // Jupiter
            -1.606149641038883_e4, // Ganymede
            -1.773273548650868_e4, // Callisto
            -1.964022059149258_e4, // Io
            -2.368456746417980_e4, // Europa
            -2.308779100011594_e3, // Saturn
            3.2720981012371810_e3, // Titan
            -1.781978737455461_e3, // Rhea
            6.1668393778760410_e2, // Iapetus
            1.2115055839547210_e3, // Dione
            -1.288492482056972_e4, // Tethys
            -1.016369833965983_e4, // Enceladus
            -6.095909686076245_e3, // Uranus
            -3.537531071429379_e3, // Titania
            -3.020488843277462_e3, // Oberon
            -9.100219608457833_e3, // Umbriel
            -1.142311981786223_e4, // Ariel
            -2.824328940406889_e2, // Neptune
            -3.388119694840936_e3, // Triton
            4.6608561593756490_e3, // Pluto
            4.5781947340292880_e3, // Charon
        ],
        vy: [
            4.2750559298809390_e0, // Sun
            -1.135209935499109_e4, // Mercury
            3.3760549848349480_e4, // Venus
            2.9634666051441470_e4, // Earth
            3.0662815580545390_e4, // Moon
            5.0945540741963760_e3, // Mars
            -7.916830005351583_e3, // Jupiter
            -1.694648069010021_e4, // Ganymede
            -1.076059119010200_e4, // Callisto
            -2.229980857866069_e4, // Io
            -6.892351344129491_e3, // Europa
            9.4736644432440930_e3, // Saturn
            9.2096018910353340_e3, // Titan
            1.6943210678687880_e4, // Rhea
            8.1780720833807090_e3, // Iapetus
            1.0290592864227970_e3, // Dione
            6.2137070454189850_e3, // Tethys
            1.8584741731956490_e4, // Enceladus
            2.8173439837442510_e3, // Uranus
            1.9184033021085590_e3, // Titania
            2.1854123916751040_e3, // Oberon
            3.9406083673346900_e3, // Umbriel
            4.0716078099974130_e3, // Ariel
            5.4625203752117270_e3, // Neptune
            2.3666896364851450_e3, // Triton
            1.8559195818375090_e3, // Pluto
            1.6892391722194860_e3, // Charon
        ],
        vz: [
            -2.231806295243432_e-1, // Sun
            -4.316244846894577_e3,  // Mercury
            -3.044460412270666_e1,  // Venus
            -2.792637945621124_e0,  // Earth
            8.0108522791542570_e1,  // Moon
            6.7337758877378380_e4,  // Mars
            2.5788395529456000_e2,  // Jupiter
            -1.751237702501642_e2,  // Ganymede
            6.5386515757815730_e1,  // Callisto
            -3.993917604807145_e2,  // Io
            1.3142984295219760_e1,  // Europa
            -7.278327685496322_e1,  // Saturn
            -4.917922364496885_e2,  // Titan
            -4.078076379784886_e3,  // Rhea
            -3.623143064349952_e2,  // Iapetus
            4.0076292058787730_e3,  // Dione
            2.4525724643706870_e3,  // Tethys
            -4.084834596826666_e3,  // Enceladus
            8.9449375049727830_e1,  // Uranus
            -2.336238626663934_e3,  // Titania
            3.5789295180543290_e2,  // Oberon
            3.4769674671325620_e3,  // Umbriel
            7.3399633252110650_e2,  // Ariel
            -1.059157030917579_e2,  // Neptune
            -2.990696721956994_e2,  // Triton
            -1.528848629659647_e3,  // Pluto
            -1.652015442732303_e3,  // Charon
        ],
        mass: [
            1.9884750_e30, // Sun
            3.3011000_e23, // Mercury
            4.8673100_e24, // Venus
            5.9721700_e24, // Earth
            7.3460000_e22, // Moon
            6.4171000_e23, // Mars
            1.8981250_e27, // Jupiter
            1.4814780_e23, // Ganymede
            1.0756600_e23, // Callisto
            8.9296000_e22, // Io
            4.7998400_e22, // Europa
            5.6831700_e26, // Saturn
            1.3451800_e23, // Titan
            2.3064854_e21, // Rhea
            1.8056591_e21, // Iapetus
            1.0954868_e21, // Dione
            6.1749590_e20, // Tethys
            1.0803180_e20, // Enceladus
            8.6809900_e25, // Uranus
            3.3380000_e21, // Titania
            3.2090000_e21, // Oberon
            1.2800000_e21, // Umbriel
            1.2500000_e21, // Ariel
            1.0240920_e26, // Neptune
            2.1389000_e22, // Triton
            1.3025000_e22, // Pluto
            1.5897000_e21, // Charon
        ],
        radius: [
            6.955000_e8, // Sun
            2.439400_e6, // Mercury
            6.051800_e6, // Venus
            6.378137_e6, // Earth
            1.738100_e6, // Moon
            3.396200_e6, // Mars
            7.148800_e7, // Jupiter
            2.632270_e6, // Ganymede
            2.410300_e6, // Callisto
            1.824200_e6, // Io
            1.560800_e6, // Europa
            5.823200_e7, // Saturn
            2.574765_e6, // Titan
            1.528090_e6, // Rhea
            1.493400_e6, // Iapetus
            5.614000_e5, // Dione
            5.310000_e5, // Tethys
            2.520400_e5, // Enceladus
            2.555660_e7, // Uranus
            7.884000_e5, // Titania
            7.614000_e5, // Oberon
            5.824000_e5, // Umbriel
            5.789000_e5, // Ariel
            2.476060_e7, // Neptune
            1.353400_e6, // Triton
            1.188300_e6, // Pluto
            6.060000_e5, // Charon
        ],
    };

    let mut iteration: usize = 0;

    let start = Instant::now();

    while iteration < step {
        for i in 0..N {
            let x_i: f64 = solsys.x[i];
            let y_i: f64 = solsys.y[i];
            let z_i: f64 = solsys.z[i];
            let mut ax_i: f64 = 0.0;
            let mut ay_i: f64 = 0.0;
            let mut az_i: f64 = 0.0;

            for j in 0..N {
                if j != i {
                    let dx: f64 = solsys.x[j] - x_i;
                    let dy: f64 = solsys.y[j] - y_i;
                    let dz: f64 = solsys.z[j] - z_i;
                    let r_2: f64 = (dx * dx) + (dy * dy) + (dz * dz);
                    let inv_r: f64 = 1.0 / r_2.sqrt();
                    let inv_r_3: f64 = inv_r * inv_r * inv_r;
                    ax_i += G * solsys.mass[j] * dx * inv_r_3;
                    ay_i += G * solsys.mass[j] * dy * inv_r_3;
                    az_i += G * solsys.mass[j] * dz * inv_r_3;
                }
            }

            solsys.vx[i] = solsys.vx[i] + (ax_i * dt);
            solsys.vy[i] = solsys.vy[i] + (ay_i * dt);
            solsys.vz[i] = solsys.vz[i] + (az_i * dt);
            solsys.x[i] = solsys.x[i] + (solsys.vx[i] * dt);
            solsys.y[i] = solsys.y[i] + (solsys.vy[i] * dt);
            solsys.z[i] = solsys.z[i] + (solsys.vz[i] * dt);
        }

        iteration += 1;
    }

    let duration = start.elapsed();

    println!("Elapsed time: {:?}", duration);
    println!("Final x[0]: {}", solsys.x[0]);
}
