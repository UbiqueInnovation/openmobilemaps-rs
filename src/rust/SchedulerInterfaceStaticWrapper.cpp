// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "SchedulerInterfaceStaticWrapper.h"
#include "cxxgen1.h"

void SchedulerInterfaceStaticWrapper::addTask(const std::shared_ptr<TaskInterface> &task) const {
    {
        auto inner = new_task_interface();
        inner->addTaskRust(task);
    }
}

void SchedulerInterfaceStaticWrapper::addTasks(const std::vector<std::shared_ptr<TaskInterface>> &tasks) const {
    {
        auto inner = new_task_interface();
        for (auto task : tasks) {
            inner->addTaskRust(task);
        }
    }
}

void SchedulerInterfaceStaticWrapper::removeTask(const std::string &id) const {
    auto inner = new_task_interface();
    inner->removeTaskRust(id);
}

void SchedulerInterfaceStaticWrapper::clear() const {
    auto inner = new_task_interface();
    inner->clearRust();
}

void SchedulerInterfaceStaticWrapper::pause() const {
    auto inner = new_task_interface();
    inner->pauseRust();
}

void SchedulerInterfaceStaticWrapper::resume() const {
    auto inner = new_task_interface();
    inner->resumeRust();
}
