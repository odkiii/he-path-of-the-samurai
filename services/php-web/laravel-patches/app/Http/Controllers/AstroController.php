<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class AstroController extends Controller
{
    public function events(Request $r)
    {
        $lat  = (float) $r->query('lat', 55.7558);
        $lon  = (float) $r->query('lon', 37.6176);
        $days = max(1, min(30, (int) $r->query('days', 7)));

        $from = now('UTC')->toDateString();
        $to   = now('UTC')->addDays($days)->toDateString();

        $appId  = env('ASTRO_APP_ID', '');
        $secret = env('ASTRO_APP_SECRET', '');
        if ($appId === '' || $secret === '') {
            return response()->json(['error' => 'Missing ASTRO_APP_ID/ASTRO_APP_SECRET'], 500);
        }

        $auth = base64_encode($appId . ':' . $secret);
        $url  = 'https://api.astronomyapi.com/api/v2/bodies/events?' . http_build_query([
            'latitude'  => $lat,
            'longitude' => $lon,
            'from'      => $from,
            'to'        => $to,
        ]);

        $ch = curl_init($url);
        curl_setopt_array($ch, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_HTTPHEADER     => [
                'Authorization: Basic ' . $auth,
                'Content-Type: application/json',
                'User-Agent: monolith-iss/1.0'
            ],
            CURLOPT_TIMEOUT        => 25,
        ]);
        $raw  = curl_exec($ch);
        $code = curl_getinfo($ch, CURLINFO_RESPONSE_CODE) ?: 0;
        $err  = curl_error($ch);
        curl_close($ch);

        if ($raw === false || $code >= 400) {
            // фоллбэк на всякий случай 
            return response()->json([
                'data' => [
                    'from' => $from,
                    'to' => $to,
                    'rows' => [
                        [
                            'body' => ['name' => 'Moon', 'type' => 'Moon'],
                            'type' => 'Phase',
                            'date' => $from,
                            'time' => '20:00:00',
                            'description' => 'Mock Event: Moon Phase'
                        ],
                        [
                            'body' => ['name' => 'Mars', 'type' => 'Planet'],
                            'type' => 'Opposition',
                            'date' => $to,
                            'time' => '22:00:00',
                            'description' => 'Mock Event: Mars Opposition'
                        ],
                        [
                            'body' => ['name' => 'Jupiter', 'type' => 'Planet'],
                            'type' => 'Conjunction',
                            'date' => now('UTC')->addDays(2)->toDateString(),
                            'time' => '05:30:00',
                            'description' => 'Mock Event: Jupiter-Saturn Conjunction'
                        ],
                        [
                            'body' => ['name' => 'Venus', 'type' => 'Planet'],
                            'type' => 'Greatest Elongation',
                            'date' => now('UTC')->addDays(4)->toDateString(),
                            'time' => '18:45:00',
                            'description' => 'Mock Event: Evening Greatest Elongation of Venus'
                        ],
                        [
                            'body' => ['name' => 'Comet C/2025 A1', 'type' => 'Comet'],
                            'type' => 'Close Approach',
                            'date' => now('UTC')->addDays(5)->toDateString(),
                            'time' => '03:00:00',
                            'description' => 'Mock Event: Close approach of Comet C/2025 A1 to Earth'
                        ],
                        [
                            'body' => ['name' => 'ISS', 'type' => 'Satellite'],
                            'type' => 'Visible Pass',
                            'date' => now('UTC')->addDays(6)->toDateString(),
                            'time' => '23:10:00',
                            'description' => 'Mock Event: Bright pass of ISS over observation point'
                        ],
                        [
                            'body' => ['name' => 'Orionids', 'type' => 'Meteor Shower'],
                            'type' => 'Peak Activity',
                            'date' => now('UTC')->addDays(7)->toDateString(),
                            'time' => '00:00:00',
                            'description' => 'Mock Event: Peak of the Orionids meteor shower'
                        ]
                    ]
                ]
            ]);
        }
        $json = json_decode($raw, true);
        return response()->json($json ?? ['raw' => $raw]);
    }
}