<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class LegacyController extends Controller
{
    public function index()
    {
        $items = DB::table('telemetry_legacy')
            ->orderBy('recorded_at', 'desc')
            ->limit(50)
            ->get();

        return view('legacy', ['items' => $items]);
    }
}