cd "$(dirname "$0")" || exit 1

echo "Building..."
if ! cargo build --quiet; then
    echo "Build failed, fix compile errors first."
    exit 1
fi

pass=0
fail=0

for f in tests/*; do
    [ -f "$f" ] || continue
    name=$(basename "$f")

    # --quiet hides cargo's own output so only the compiler's messages show
    output=$(cargo run --quiet -- "$f" 2>&1)
    code=$?

    case "$name" in
        *invalid*|*bad*|*fail*) expect_fail=1 ;;
        *)                      expect_fail=0 ;;
    esac

    if [ "$expect_fail" -eq 0 ] && [ "$code" -eq 0 ]; then
        echo "PASS  $name"
        pass=$((pass + 1))
    elif [ "$expect_fail" -eq 1 ] && [ "$code" -ne 0 ]; then
        echo "PASS  $name (correctly rejected)"
        pass=$((pass + 1))
    else
        echo "FAIL  $name (exit code $code, expected $([ $expect_fail -eq 1 ] && echo nonzero || echo 0))"
        echo "$output" | sed 's/^/        /'
        fail=$((fail + 1))
    fi
done

echo
echo "Passed: $pass   Failed: $fail"
[ "$fail" -eq 0 ]