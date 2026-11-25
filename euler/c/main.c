// N-Body gravity simulation.
// Euler integrator.
// Single thread. Auto-optimizations.

// Meters, Kilograms, seconds.

#include <stdio.h>
#include <time.h>
#include <math.h>

#define G 6.6743e-11
#define N 27

struct SBodies {
    double          x[N];
    double          y[N];
    double          z[N];
    double          vx[N];
    double          vy[N];
    double          vz[N];
    double          mass[N];
    double          radius[N];
};

int main(void) {
    double dt = 1.0;
    unsigned int step = 10000000;

    // Positions and velocities from NASA Horizons data for 26-09-2026 at 00:00:00.0000 TDB. Solar System Barycenter.
    // Masses and radii from Wikipedia.
    struct SBodies solsys = {
        .x = {
            -1.908487414960836e8,      // Sun
            -1.991359984355149e10,     // Mercury
            1.0499520185477800e11,     // Venus
            1.4985628126514220e11,     // Earth
            1.5023609453631250e11,     // Moon
            2.8860581412857820e10,     // Mars
            -5.180202934546471e11,     // Jupiter
            -5.189142422390277e11,     // Ganymede
            -5.186876952078271e11,     // Callisto
            -5.183728483681735e11,     // Io
            -5.179648053059427e11,     // Europa
            1.3859665737388790e12,     // Saturn
            1.3859232009978700e12,     // Titan
            1.3864903674899490e12,     // Rhea
            1.3845642172171870e12,     // Iapetus
            1.3856145802590930e12,     // Dione
            1.3858614077392210e12,     // Tethys
            1.3861512250178400e12,     // Enceladus
            1.3384663788023680e12,     // Uranus
            1.3387625976246830e12,     // Titania
            1.3384329634074950e12,     // Oberon
            1.3382712547875050e12,     // Umbriel
            1.3384388872278930e12,     // Ariel
            4.4635734869182730e12,     // Neptune
            4.4637402686055140e12,     // Triton
            2.9844087233868480e12,     // Pluto
            2.9844211608676770e12,     // Charon
        },
        .y = {
            -7.708483695236095e8,      // Sun
            -6.760385185792711e10,     // Mercury
            -2.788385821145211e10,     // Venus
            6.7058513945656970e9,      // Earth
            6.6647035007779940e9,      // Moon
            2.2955055657416670e11,     // Mars
            6.0062292119704140e11,     // Jupiter
            6.0121608320380090e11,     // Ganymede
            6.0238550013893700e11,     // Callisto
            6.0085603570881390e11,     // Io
            6.0129461316842930e11,     // Europa
            2.5943481015740690e11,     // Saturn
            2.5835715346153020e11,     // Titan
            2.5938564929816200e11,     // Rhea
            2.5625583252272930e11,     // Iapetus
            2.5933012346008860e11,     // Dione
            2.5968136674069510e11,     // Tethys
            2.5955851762713360e11,     // Enceladus
            2.5813516616229240e12,     // Uranus
            2.5813315085676570e12,     // Titania
            2.5814407655852080e12,     // Oberon
            2.5813691458371830e12,     // Umbriel
            2.5813316184433400e12,     // Ariel
            2.0333943854360820e11,     // Neptune
            2.0318924629098220e11,     // Triton
            -4.395251268785082e12,     // Pluto
            -4.395246760028754e12,     // Charon
        },
        .z = {
            1.4257422826508060e7,      // Sun
            -3.638708605360989e9,      // Mercury
            -6.427335092098974e9,      // Venus
            -1.490024972805753e6,      // Earth
            1.2975563339395450e7,      // Moon
            4.1286053301584870e9,      // Mars
            9.1017943978388010e9,      // Jupiter
            9.1113992933885750e9,      // Ganymede
            9.1480161320003570e9,      // Callisto
            9.1051800271157030e9,      // Io
            9.1231908979673980e9,      // Europa
            -5.969501488151904e10,     // Saturn
            -5.913502531847332e10,     // Titan
            -5.971803461256965e10,     // Rhea
            -5.868229575249475e10,     // Iapetus
            -5.960622956277069e10,     // Dione
            -5.981740379289320e10,     // Tethys
            -5.977769123482867e10,     // Enceladus
            -7.753248151706457e9,      // Uranus
            -7.432641617763758e9,      // Titania
            -7.177519974507570e9,      // Oberon
            -7.933368431657553e9,      // Umbriel
            -7.941061654786944e9,      // Ariel
            -1.070549795671838e11,     // Neptune
            -1.073297692001680e11,     // Triton
            -3.929487081633263e11,     // Pluto
            -3.929631619999628e11,     // Charon
        },
        .vx = {
            1.0522744354555900e1,      // Sun
            3.6949711575351460e4,      // Mercury
            8.5726349624921720e3,      // Venus
            -1.815389568008479e3,      // Earth
            -1.756791179725980e3,      // Moon
            -2.311029638337283e4,      // Mars
            -1.004921344065230e4,      // Jupiter
            -1.606149641038883e4,      // Ganymede
            -1.773273548650868e4,      // Callisto
            -1.964022059149258e4,      // Io
            -2.368456746417980e4,      // Europa
            -2.308779100011594e3,      // Saturn
            3.2720981012371810e3,      // Titan
            -1.781978737455461e3,      // Rhea
            6.1668393778760410e2,      // Iapetus
            1.2115055839547210e3,      // Dione
            -1.288492482056972e4,      // Tethys
            -1.016369833965983e4,      // Enceladus
            -6.095909686076245e3,      // Uranus
            -3.537531071429379e3,      // Titania
            -3.020488843277462e3,      // Oberon
            -9.100219608457833e3,      // Umbriel
            -1.142311981786223e4,      // Ariel
            -2.824328940406889e2,      // Neptune
            -3.388119694840936e3,      // Triton
            4.6608561593756490e3,      // Pluto
            4.5781947340292880e3,      // Charon
        },
        .vy = {
            4.2750559298809390e0,      // Sun
            -1.135209935499109e4,      // Mercury
            3.3760549848349480e4,      // Venus
            2.9634666051441470e4,      // Earth
            3.0662815580545390e4,      // Moon
            5.0945540741963760e3,      // Mars
            -7.916830005351583e3,      // Jupiter
            -1.694648069010021e4,      // Ganymede
            -1.076059119010200e4,      // Callisto
            -2.229980857866069e4,      // Io
            -6.892351344129491e3,      // Europa
            9.4736644432440930e3,      // Saturn
            9.2096018910353340e3,      // Titan
            1.6943210678687880e4,      // Rhea
            8.1780720833807090e3,      // Iapetus
            1.0290592864227970e3,      // Dione
            6.2137070454189850e3,      // Tethys
            1.8584741731956490e4,      // Enceladus
            2.8173439837442510e3,      // Uranus
            1.9184033021085590e3,      // Titania
            2.1854123916751040e3,      // Oberon
            3.9406083673346900e3,      // Umbriel
            4.0716078099974130e3,      // Ariel
            5.4625203752117270e3,      // Neptune
            2.3666896364851450e3,      // Triton
            1.8559195818375090e3,      // Pluto
            1.6892391722194860e3,      // Charon
        },
        .vz = {
            -2.231806295243432e-1,     // Sun
            -4.316244846894577e3,      // Mercury
            -3.044460412270666e1,      // Venus
            -2.792637945621124e0,      // Earth
            8.0108522791542570e1,      // Moon
            6.7337758877378380e4,      // Mars
            2.5788395529456000e2,      // Jupiter
            -1.751237702501642e2,      // Ganymede
            6.5386515757815730e1,      // Callisto
            -3.993917604807145e2,      // Io
            1.3142984295219760e1,      // Europa
            -7.278327685496322e1,      // Saturn
            -4.917922364496885e2,      // Titan
            -4.078076379784886e3,      // Rhea
            -3.623143064349952e2,      // Iapetus
            4.0076292058787730e3,      // Dione
            2.4525724643706870e3,      // Tethys
            -4.084834596826666e3,      // Enceladus
            8.9449375049727830e1,      // Uranus
            -2.336238626663934e3,      // Titania
            3.5789295180543290e2,      // Oberon
            3.4769674671325620e3,      // Umbriel
            7.3399633252110650e2,      // Ariel
            -1.059157030917579e2,      // Neptune
            -2.990696721956994e2,      // Triton
            -1.528848629659647e3,      // Pluto
            -1.652015442732303e3,      // Charon
        },
        .mass = {
            1.9884750e30,      // Sun
            3.3011000e23,      // Mercury
            4.8673100e24,      // Venus
            5.9721700e24,      // Earth
            7.3460000e22,      // Moon
            6.4171000e23,      // Mars
            1.8981250e27,      // Jupiter
            1.4814780e23,      // Ganymede
            1.0756600e23,      // Callisto
            8.9296000e22,      // Io
            4.7998400e22,      // Europa
            5.6831700e26,      // Saturn
            1.3451800e23,      // Titan
            2.3064854e21,      // Rhea
            1.8056591e21,      // Iapetus
            1.0954868e21,      // Dione
            6.1749590e20,      // Tethys
            1.0803180e20,      // Enceladus
            8.6809900e25,      // Uranus
            3.3380000e21,      // Titania
            3.2090000e21,      // Oberon
            1.2800000e21,      // Umbriel
            1.2500000e21,      // Ariel
            1.0240920e26,      // Neptune
            2.1389000e22,      // Triton
            1.3025000e22,      // Pluto
            1.5897000e21,      // Charon
        },
        .radius = {
            6.955000e8,    // Sun
            2.439400e6,    // Mercury
            6.051800e6,    // Venus
            6.378137e6,    // Earth
            1.738100e6,    // Moon
            3.396200e6,    // Mars
            7.148800e7,    // Jupiter
            2.632270e6,    // Ganymede
            2.410300e6,    // Callisto
            1.824200e6,    // Io
            1.560800e6,    // Europa
            5.823200e7,    // Saturn
            2.574765e6,    // Titan
            1.528090e6,    // Rhea
            1.493400e6,    // Iapetus
            5.614000e5,    // Dione
            5.310000e5,    // Tethys
            2.520400e5,    // Enceladus
            2.555660e7,    // Uranus
            7.884000e5,    // Titania
            7.614000e5,    // Oberon
            5.824000e5,    // Umbriel
            5.789000e5,    // Ariel
            2.476060e7,    // Neptune
            1.353400e6,    // Triton
            1.188300e6,    // Pluto
            6.060000e5,    // Charon
        }
    };

    unsigned int iteration = 0;

    clock_t start = clock();

    while (iteration < step) {
        for (unsigned int i = 0; i < N; i = i + 1) {
            double x_i = solsys.x[i];
            double y_i = solsys.y[i];
            double z_i = solsys.z[i];
            double ax_i = 0.0;
            double ay_i = 0.0;
            double az_i = 0.0;

            for (unsigned int j = 0; j < N; j = j + 1) {
                if (j != i) {
                    double dx = solsys.x[j] - x_i;
                    double dy = solsys.y[j] - y_i;
                    double dz = solsys.z[j] - z_i;
                    double r_2 = (dx * dx) + (dy * dy) + (dz * dz);
                    double inv_r = 1.0 / sqrt(r_2);
                    double inv_r_3 = inv_r * inv_r * inv_r;
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

        iteration = iteration + 1;
    }

    clock_t end = clock();
    double duration = ((double) (end - start)) / CLOCKS_PER_SEC;

    printf("Execution time: %f seconds\n", duration);
    printf("final x[0]: %f\n", solsys.x[0]);

    return 0;
}
