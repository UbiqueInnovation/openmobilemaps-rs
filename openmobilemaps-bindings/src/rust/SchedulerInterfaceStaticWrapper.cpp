// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#include "SchedulerInterfaceStaticWrapper.h"
#include "cxxgen1.h"

SchedulerInterfaceStaticWrapper::SchedulerInterfaceStaticWrapper(SchedulerInterfaceRust *ptr) : rustBox(::rust::Box<SchedulerInterfaceRust>::from_raw(ptr))
{
}

void SchedulerInterfaceStaticWrapper::addTask(const std::shared_ptr<TaskInterface> &task)
{
    this->rustBox->addTaskRust(task);
    // {
    //     auto inner = new_task_interface();
    //     inner->addTaskRust(task);
    // }
}

void SchedulerInterfaceStaticWrapper::addTasks(const std::vector<std::shared_ptr<TaskInterface>> &tasks)
{
    {
        // auto inner = new_task_interface();
        for (auto task : tasks)
        {
            this->rustBox->addTaskRust(task);
        }
    }
}

void SchedulerInterfaceStaticWrapper::removeTask(const std::string &id)
{
    // auto inner = new_task_interface();
    this->rustBox->removeTaskRust(id);
}

void SchedulerInterfaceStaticWrapper::clear()
{
    // auto inner = new_task_interface();
    this->rustBox->clearRust();
}

void SchedulerInterfaceStaticWrapper::pause()
{
    // auto inner = new_task_interface();
    this->rustBox->pauseRust();
}

void SchedulerInterfaceStaticWrapper::resume()
{
    // auto inner = new_task_interface();
    this->rustBox->resumeRust();
}
