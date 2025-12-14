<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use Illuminate\Support\Facades\DB;

class SearchController extends Controller
{
    public function index(Request $request)
    {
        $query = $request->input('q');
        $filter = $request->input('filter', 'all');

        $results = [];

        if ($query) {
            if ($filter === 'all' || $filter === 'osdr') {
                $osdr = DB::table('osdr_items')
                    ->where('title', 'ilike', "%{$query}%")
                    ->orWhere('dataset_id', 'ilike', "%{$query}%")
                    ->limit(20)
                    ->get()
                    ->map(fn($item) => ['type' => 'OSDR', 'title' => $item->title, 'link' => '/osdr?id='.$item->id]);
                $results = array_merge($results, $osdr->toArray());
            }
        }

        return view('search', ['results' => $results, 'query' => $query, 'filter' => $filter]);
    }
}