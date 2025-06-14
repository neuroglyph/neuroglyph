# SPDX-License-Identifier: Apache-2.0
# Docker Guard - Prevents builds/tests outside Docker

# Check if we're inside Docker
ifndef DOCKER_CONTAINER
    ifeq ($(wildcard /.dockerenv),)
        # Not in Docker! Check for other Docker indicators
        ifeq ($(shell grep -q docker /proc/1/cgroup 2>/dev/null && echo yes || echo no),no)
            # Check if running in GitHub Actions (for cross-platform testing)
            ifndef GITHUB_ACTIONS
                $(error ❌ FATAL: You must run this inside Docker! Use 'make' from the root directory or 'docker compose run dev make')
            else
                $(info ⚠️  Running in GitHub Actions - Docker check bypassed for cross-platform testing)
            endif
        endif
    endif
endif

# If we got here, we're in Docker (or explicitly bypassed with DOCKER_CONTAINER=1)
$(info ✅ Running inside Docker container)