// Copyright (c) 2023 Ubique Innovation AG <https://www.ubique.ch>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#pragma once
#include "SchedulerInterface.h"
// #include "cxxgen.h"
struct SchedulerInterfaceRust;

class SchedulerInterfaceStaticWrapper : public SchedulerInterface {
  public:
    virtual void addTask(const std::shared_ptr<TaskInterface> &task) const override;

    virtual void addTasks(const std::vector<std::shared_ptr<TaskInterface>> &tasks) const override;

    virtual void removeTask(const std::string &id) const override;

    virtual void clear() const override;

    virtual void pause() const override;

    virtual void resume() const override;

    static std::shared_ptr<SchedulerInterface> as_scheduler_interface_shared_ptr(std::unique_ptr<SchedulerInterface> ptr) {
        return ptr;
    }
};
