<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;

class OsdrController extends Controller
{
    public function index(Request $request)
    {
        $limit = $request->query('limit', '50');
        $sort  = $request->query('sort', 'inserted_at');
        $dir   = $request->query('dir', 'desc');
        $filterCol = $request->query('filter_col');
        $filterVal = $request->query('filter_val');

        $base  = getenv('RUST_BASE') ?: 'http://rust_iss:3000';

        $json  = @file_get_contents($base.'/osdr/list?limit='.$limit);
        $data  = $json ? json_decode($json, true) : ['items' => []];
        $items = $data['items'] ?? [];

        $items = $this->flattenOsdr($items);

        // фильтрация
        if ($filterCol && $filterVal) {
            $items = array_filter($items, function($item) use ($filterCol, $filterVal) {
                return isset($item[$filterCol]) && stripos((string)$item[$filterCol], $filterVal) !== false;
            });
        }

        // соритировка
        usort($items, function($a, $b) use ($sort, $dir) {
            $valA = $a[$sort] ?? '';
            $valB = $b[$sort] ?? '';
            if ($valA == $valB) return 0;
            $res = ($valA < $valB) ? -1 : 1;
            return $dir === 'desc' ? -$res : $res;
        });

        return view('osdr', [
            'items' => $items,
            'src'   => $base.'/osdr/list?limit='.$limit,
            'sort'  => $sort,
            'dir'   => $dir,
        ]);
    }

    /** Преобразует данные вида {"OSD-1": {...}, "OSD-2": {...}} в плоский список */
    private function flattenOsdr(array $items): array
    {
        $out = [];
        foreach ($items as $row) {
            $raw = $row['raw'] ?? [];
            if (is_array($raw) && $this->looksOsdrDict($raw)) {
                foreach ($raw as $k => $v) {
                    if (!is_array($v)) continue;
                    $rest = $v['REST_URL'] ?? $v['rest_url'] ?? $v['rest'] ?? null;
                    $title = $v['title'] ?? $v['name'] ?? null;
                    if (!$title && is_string($rest)) {
                        // запасной вариант: последний сегмент URL как подпись
                        $title = basename(rtrim($rest, '/'));
                    }
                    $out[] = [
                        'id'          => $row['id'],
                        'dataset_id'  => $k,
                        'title'       => $title,
                        'status'      => $row['status'] ?? null,
                        'updated_at'  => $row['updated_at'] ?? null,
                        'inserted_at' => $row['inserted_at'] ?? null,
                        'rest_url'    => $rest,
                        'raw'         => $v,
                    ];
                }
            } else {
                // обычная строка — просто прокинем REST_URL если найдётся
                $row['rest_url'] = is_array($raw) ? ($raw['REST_URL'] ?? $raw['rest_url'] ?? null) : null;
                $out[] = $row;
            }
        }
        return $out;
    }

    private function looksOsdrDict(array $raw): bool
    {
        // словарь ключей "OSD-xxx" ИЛИ значения содержат REST_URL
        foreach ($raw as $k => $v) {
            if (is_string($k) && str_starts_with($k, 'OSD-')) return true;
            if (is_array($v) && (isset($v['REST_URL']) || isset($v['rest_url']))) return true;
        }
        return false;
    }
}