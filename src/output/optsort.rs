pub fn optsort2(a: [usize; 2usize]) -> [usize; 2usize] {
    let mut p = [0usize, 1usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [p[1usize], p[0usize]];
    }
    return [a[p[0usize]], a[p[1usize]]];
}

pub fn optsort3(a: [usize; 3usize]) -> [usize; 3usize] {
    let mut p = [0usize, 1usize, 2usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [p[1usize], p[0usize], p[2usize]];
    }
    if a[p[0usize]] < a[p[2usize]] {
        if a[p[1usize]] > a[p[2usize]] {
            p = [p[0usize], p[2usize], p[1usize]];
        }
        return [a[p[0usize]], a[p[1usize]], a[p[2usize]]];
    } else {
        return [a[p[2usize]], a[p[0usize]], a[p[1usize]]];
    }
}

pub fn optsort4(a: [usize; 4usize]) -> [usize; 4usize] {
    let mut p = [0usize, 1usize, 2usize, 3usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [p[1usize], p[0usize], p[2usize], p[3usize]];
    }
    if a[p[0usize]] < a[p[2usize]] {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[1usize]] > a[p[2usize]] {
                p = [p[0usize], p[2usize], p[1usize], p[3usize]];
            }
            if a[p[1usize]] < a[p[3usize]] {
                if a[p[2usize]] > a[p[3usize]] {
                    p = [p[0usize], p[1usize], p[3usize], p[2usize]];
                }
                return [a[p[0usize]], a[p[1usize]], a[p[2usize]], a[p[3usize]]];
            } else {
                return [a[p[0usize]], a[p[3usize]], a[p[1usize]], a[p[2usize]]];
            }
        } else {
            if a[p[1usize]] > a[p[2usize]] {
                p = [p[0usize], p[2usize], p[1usize], p[3usize]];
            }
            return [a[p[3usize]], a[p[0usize]], a[p[1usize]], a[p[2usize]]];
        }
    } else {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[1usize]] > a[p[3usize]] {
                p = [p[0usize], p[3usize], p[2usize], p[1usize]];
            }
            return [a[p[2usize]], a[p[0usize]], a[p[1usize]], a[p[3usize]]];
        } else {
            if a[p[2usize]] > a[p[3usize]] {
                p = [p[0usize], p[1usize], p[3usize], p[2usize]];
            }
            return [a[p[2usize]], a[p[3usize]], a[p[0usize]], a[p[1usize]]];
        }
    }
}

pub fn optsort5(a: [usize; 5usize]) -> [usize; 5usize] {
    let mut p = [0usize, 1usize, 2usize, 3usize, 4usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [p[1usize], p[0usize], p[2usize], p[3usize], p[4usize]];
    }
    if a[p[0usize]] < a[p[2usize]] {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[1usize]] > a[p[2usize]] {
                    p = [p[0usize], p[2usize], p[1usize], p[3usize], p[4usize]];
                }
                if a[p[1usize]] < a[p[3usize]] {
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [p[0usize], p[1usize], p[3usize], p[2usize], p[4usize]];
                        }
                        if a[p[2usize]] < a[p[4usize]] {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [p[0usize], p[1usize], p[2usize], p[4usize], p[3usize]];
                            }
                            return [
                                a[p[0usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                                a[p[4usize]],
                            ];
                        } else {
                            return [
                                a[p[0usize]],
                                a[p[1usize]],
                                a[p[4usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                            ];
                        }
                    } else {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [p[0usize], p[1usize], p[3usize], p[2usize], p[4usize]];
                        }
                        return [
                            a[p[0usize]],
                            a[p[4usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                            a[p[3usize]],
                        ];
                    }
                } else {
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[2usize]] > a[p[4usize]] {
                            p = [p[0usize], p[1usize], p[4usize], p[3usize], p[2usize]];
                        }
                        return [
                            a[p[0usize]],
                            a[p[3usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                            a[p[4usize]],
                        ];
                    } else {
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [p[0usize], p[1usize], p[2usize], p[4usize], p[3usize]];
                        }
                        return [
                            a[p[0usize]],
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    }
                }
            } else {
                if a[p[1usize]] > a[p[2usize]] {
                    p = [p[0usize], p[2usize], p[1usize], p[3usize], p[4usize]];
                }
                if a[p[1usize]] < a[p[3usize]] {
                    if a[p[2usize]] > a[p[3usize]] {
                        p = [p[0usize], p[1usize], p[3usize], p[2usize], p[4usize]];
                    }
                    return [
                        a[p[4usize]],
                        a[p[0usize]],
                        a[p[1usize]],
                        a[p[2usize]],
                        a[p[3usize]],
                    ];
                } else {
                    return [
                        a[p[4usize]],
                        a[p[0usize]],
                        a[p[3usize]],
                        a[p[1usize]],
                        a[p[2usize]],
                    ];
                }
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[1usize]] > a[p[2usize]] {
                    p = [p[0usize], p[2usize], p[1usize], p[3usize], p[4usize]];
                }
                if a[p[1usize]] < a[p[4usize]] {
                    if a[p[2usize]] > a[p[4usize]] {
                        p = [p[0usize], p[1usize], p[4usize], p[3usize], p[2usize]];
                    }
                    return [
                        a[p[3usize]],
                        a[p[0usize]],
                        a[p[1usize]],
                        a[p[2usize]],
                        a[p[4usize]],
                    ];
                } else {
                    return [
                        a[p[3usize]],
                        a[p[0usize]],
                        a[p[4usize]],
                        a[p[1usize]],
                        a[p[2usize]],
                    ];
                }
            } else {
                if a[p[1usize]] > a[p[2usize]] {
                    p = [p[0usize], p[2usize], p[1usize], p[3usize], p[4usize]];
                }
                if a[p[3usize]] > a[p[4usize]] {
                    p = [p[0usize], p[1usize], p[2usize], p[4usize], p[3usize]];
                }
                return [
                    a[p[3usize]],
                    a[p[4usize]],
                    a[p[0usize]],
                    a[p[1usize]],
                    a[p[2usize]],
                ];
            }
        }
    } else {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[1usize]] > a[p[3usize]] {
                    p = [p[0usize], p[3usize], p[2usize], p[1usize], p[4usize]];
                }
                if a[p[1usize]] < a[p[4usize]] {
                    if a[p[3usize]] > a[p[4usize]] {
                        p = [p[0usize], p[1usize], p[2usize], p[4usize], p[3usize]];
                    }
                    return [
                        a[p[2usize]],
                        a[p[0usize]],
                        a[p[1usize]],
                        a[p[3usize]],
                        a[p[4usize]],
                    ];
                } else {
                    return [
                        a[p[2usize]],
                        a[p[0usize]],
                        a[p[4usize]],
                        a[p[1usize]],
                        a[p[3usize]],
                    ];
                }
            } else {
                if a[p[1usize]] > a[p[3usize]] {
                    p = [p[0usize], p[3usize], p[2usize], p[1usize], p[4usize]];
                }
                if a[p[2usize]] > a[p[4usize]] {
                    p = [p[0usize], p[1usize], p[4usize], p[3usize], p[2usize]];
                }
                return [
                    a[p[2usize]],
                    a[p[4usize]],
                    a[p[0usize]],
                    a[p[1usize]],
                    a[p[3usize]],
                ];
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[1usize]] > a[p[4usize]] {
                    p = [p[0usize], p[4usize], p[2usize], p[3usize], p[1usize]];
                }
                if a[p[2usize]] > a[p[3usize]] {
                    p = [p[0usize], p[1usize], p[3usize], p[2usize], p[4usize]];
                }
                return [
                    a[p[2usize]],
                    a[p[3usize]],
                    a[p[0usize]],
                    a[p[1usize]],
                    a[p[4usize]],
                ];
            } else {
                if a[p[2usize]] > a[p[3usize]] {
                    p = [p[0usize], p[1usize], p[3usize], p[2usize], p[4usize]];
                }
                if a[p[2usize]] < a[p[4usize]] {
                    if a[p[3usize]] > a[p[4usize]] {
                        p = [p[0usize], p[1usize], p[2usize], p[4usize], p[3usize]];
                    }
                    return [
                        a[p[2usize]],
                        a[p[3usize]],
                        a[p[4usize]],
                        a[p[0usize]],
                        a[p[1usize]],
                    ];
                } else {
                    return [
                        a[p[4usize]],
                        a[p[2usize]],
                        a[p[3usize]],
                        a[p[0usize]],
                        a[p[1usize]],
                    ];
                }
            }
        }
    }
}

pub fn optsort6(a: [usize; 6usize]) -> [usize; 6usize] {
    let mut p = [0usize, 1usize, 2usize, 3usize, 4usize, 5usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [p[1usize], p[0usize], p[2usize], p[3usize], p[4usize], p[5usize]];
    }
    if a[p[0usize]] < a[p[2usize]] {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[3usize]] {
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[4usize]] {
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                            ];
                                        }
                                        if a[p[3usize]] < a[p[5usize]] {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                            ];
                                        } else {
                                            return [
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[5usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[0usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[3usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[5usize],
                                                p[4usize],
                                                p[3usize],
                                            ];
                                        }
                                        return [
                                            a[p[0usize]],
                                            a[p[1usize]],
                                            a[p[4usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[0usize]],
                                            a[p[1usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[4usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[4usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[5usize]] {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                        ];
                                    }
                                    return [
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[5usize]] {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[5usize]] {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                    ];
                                }
                            }
                        }
                    }
                } else {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[3usize]] {
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[4usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                return [
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[5usize]],
                                a[p[0usize]],
                                a[p[4usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                            ];
                        }
                    } else {
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[5usize]],
                                a[p[0usize]],
                                a[p[3usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[4usize]],
                            ];
                        } else {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[5usize]],
                                a[p[0usize]],
                                a[p[3usize]],
                                a[p[4usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                            ];
                        }
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[3usize]] {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                return [
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[4usize]],
                                a[p[0usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                            ];
                        }
                    } else {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[5usize],
                                    p[3usize],
                                    p[4usize],
                                    p[2usize],
                                ];
                            }
                            return [
                                a[p[4usize]],
                                a[p[0usize]],
                                a[p[3usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[5usize]],
                            ];
                        } else {
                            if a[p[3usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[5usize],
                                    p[4usize],
                                    p[3usize],
                                ];
                            }
                            return [
                                a[p[4usize]],
                                a[p[0usize]],
                                a[p[3usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                            ];
                        }
                    }
                } else {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[3usize]] {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                            ];
                        }
                        if a[p[4usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[3usize],
                                p[5usize],
                                p[4usize],
                            ];
                        }
                        return [
                            a[p[4usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                            a[p[3usize]],
                        ];
                    } else {
                        if a[p[4usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[3usize],
                                p[5usize],
                                p[4usize],
                            ];
                        }
                        return [
                            a[p[4usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[3usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    }
                }
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                return [
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[3usize]],
                                a[p[0usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[4usize]],
                            ];
                        }
                    } else {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[5usize],
                                    p[3usize],
                                    p[4usize],
                                    p[2usize],
                                ];
                            }
                            return [
                                a[p[3usize]],
                                a[p[0usize]],
                                a[p[4usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                                a[p[5usize]],
                            ];
                        } else {
                            if a[p[4usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[3usize],
                                    p[5usize],
                                    p[4usize],
                                ];
                            }
                            return [
                                a[p[3usize]],
                                a[p[0usize]],
                                a[p[4usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[2usize]],
                            ];
                        }
                    }
                } else {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[2usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[4usize],
                                p[3usize],
                                p[2usize],
                                p[5usize],
                            ];
                        }
                        if a[p[3usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[5usize],
                                p[4usize],
                                p[3usize],
                            ];
                        }
                        return [
                            a[p[3usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                            a[p[4usize]],
                        ];
                    } else {
                        if a[p[3usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[5usize],
                                p[4usize],
                                p[3usize],
                            ];
                        }
                        return [
                            a[p[3usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[4usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[5usize]] {
                        if a[p[2usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[5usize],
                                p[3usize],
                                p[4usize],
                                p[2usize],
                            ];
                        }
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[4usize],
                                p[3usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                            a[p[5usize]],
                        ];
                    } else {
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[4usize],
                                p[3usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[5usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    }
                } else {
                    if a[p[1usize]] > a[p[2usize]] {
                        p = [
                            p[0usize],
                            p[2usize],
                            p[1usize],
                            p[3usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[3usize]] > a[p[4usize]] {
                        p = [
                            p[0usize],
                            p[1usize],
                            p[2usize],
                            p[4usize],
                            p[3usize],
                            p[5usize],
                        ];
                    }
                    if a[p[3usize]] < a[p[5usize]] {
                        if a[p[4usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[3usize],
                                p[5usize],
                                p[4usize],
                            ];
                        }
                        return [
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    } else {
                        return [
                            a[p[5usize]],
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[2usize]],
                        ];
                    }
                }
            }
        }
    } else {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[3usize],
                            p[2usize],
                            p[1usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                return [
                                    a[p[2usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[2usize]],
                                a[p[0usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[3usize]],
                                a[p[4usize]],
                            ];
                        }
                    } else {
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[3usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[5usize],
                                    p[4usize],
                                    p[3usize],
                                ];
                            }
                            return [
                                a[p[2usize]],
                                a[p[0usize]],
                                a[p[4usize]],
                                a[p[1usize]],
                                a[p[3usize]],
                                a[p[5usize]],
                            ];
                        } else {
                            if a[p[4usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[3usize],
                                    p[5usize],
                                    p[4usize],
                                ];
                            }
                            return [
                                a[p[2usize]],
                                a[p[0usize]],
                                a[p[4usize]],
                                a[p[5usize]],
                                a[p[1usize]],
                                a[p[3usize]],
                            ];
                        }
                    }
                } else {
                    if a[p[1usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[3usize],
                            p[2usize],
                            p[1usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[4usize]] {
                        if a[p[2usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[5usize],
                                p[3usize],
                                p[4usize],
                                p[2usize],
                            ];
                        }
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[4usize],
                                p[3usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                            a[p[4usize]],
                        ];
                    } else {
                        if a[p[2usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[5usize],
                                p[3usize],
                                p[4usize],
                                p[2usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[4usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                        ];
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[3usize],
                            p[2usize],
                            p[1usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[5usize]] {
                        if a[p[2usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[4usize],
                                p[3usize],
                                p[2usize],
                                p[5usize],
                            ];
                        }
                        if a[p[3usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[5usize],
                                p[4usize],
                                p[3usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                            a[p[5usize]],
                        ];
                    } else {
                        if a[p[2usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[4usize],
                                p[3usize],
                                p[2usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[5usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                        ];
                    }
                } else {
                    if a[p[1usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[3usize],
                            p[2usize],
                            p[1usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] > a[p[4usize]] {
                        p = [
                            p[0usize],
                            p[1usize],
                            p[4usize],
                            p[3usize],
                            p[2usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] < a[p[5usize]] {
                        if a[p[4usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[3usize],
                                p[5usize],
                                p[4usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[4usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                        ];
                    } else {
                        return [
                            a[p[5usize]],
                            a[p[2usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[3usize]],
                        ];
                    }
                }
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[4usize]] {
                        p = [
                            p[0usize],
                            p[4usize],
                            p[2usize],
                            p[3usize],
                            p[1usize],
                            p[5usize],
                        ];
                    }
                    if a[p[1usize]] < a[p[5usize]] {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                            ];
                        }
                        if a[p[4usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[3usize],
                                p[5usize],
                                p[4usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[4usize]],
                            a[p[5usize]],
                        ];
                    } else {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[0usize]],
                            a[p[5usize]],
                            a[p[1usize]],
                            a[p[4usize]],
                        ];
                    }
                } else {
                    if a[p[1usize]] > a[p[4usize]] {
                        p = [
                            p[0usize],
                            p[4usize],
                            p[2usize],
                            p[3usize],
                            p[1usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[1usize],
                            p[3usize],
                            p[2usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] < a[p[5usize]] {
                        if a[p[3usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[5usize],
                                p[4usize],
                                p[3usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[5usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[4usize]],
                        ];
                    } else {
                        return [
                            a[p[5usize]],
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[4usize]],
                        ];
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[1usize]] > a[p[5usize]] {
                        p = [
                            p[0usize],
                            p[5usize],
                            p[2usize],
                            p[3usize],
                            p[4usize],
                            p[1usize],
                        ];
                    }
                    if a[p[2usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[1usize],
                            p[3usize],
                            p[2usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] < a[p[4usize]] {
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[4usize],
                                p[3usize],
                                p[5usize],
                            ];
                        }
                        return [
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[4usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[5usize]],
                        ];
                    } else {
                        return [
                            a[p[4usize]],
                            a[p[2usize]],
                            a[p[3usize]],
                            a[p[0usize]],
                            a[p[1usize]],
                            a[p[5usize]],
                        ];
                    }
                } else {
                    if a[p[2usize]] > a[p[3usize]] {
                        p = [
                            p[0usize],
                            p[1usize],
                            p[3usize],
                            p[2usize],
                            p[4usize],
                            p[5usize],
                        ];
                    }
                    if a[p[2usize]] < a[p[4usize]] {
                        if a[p[2usize]] < a[p[5usize]] {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                ];
                            } else {
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                ];
                            }
                            return [
                                a[p[5usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                                a[p[4usize]],
                                a[p[0usize]],
                                a[p[1usize]],
                            ];
                        }
                    } else {
                        if a[p[2usize]] < a[p[5usize]] {
                            if a[p[3usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[5usize],
                                    p[4usize],
                                    p[3usize],
                                ];
                            }
                            return [
                                a[p[4usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                                a[p[5usize]],
                                a[p[0usize]],
                                a[p[1usize]],
                            ];
                        } else {
                            if a[p[4usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[3usize],
                                    p[5usize],
                                    p[4usize],
                                ];
                            }
                            return [
                                a[p[4usize]],
                                a[p[5usize]],
                                a[p[2usize]],
                                a[p[3usize]],
                                a[p[0usize]],
                                a[p[1usize]],
                            ];
                        }
                    }
                }
            }
        }
    }
}

pub fn optsort7(a: [usize; 7usize]) -> [usize; 7usize] {
    let mut p = [0usize, 1usize, 2usize, 3usize, 4usize, 5usize, 6usize];
    if a[p[0usize]] > a[p[1usize]] {
        p = [
            p[1usize],
            p[0usize],
            p[2usize],
            p[3usize],
            p[4usize],
            p[5usize],
            p[6usize],
        ];
    }
    if a[p[0usize]] < a[p[2usize]] {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[4usize]] {
                                            if a[p[2usize]] < a[p[5usize]] {
                                                if a[p[2usize]] < a[p[6usize]] {
                                                    if a[p[3usize]] > a[p[4usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[3usize]] < a[p[5usize]] {
                                                        if a[p[3usize]] < a[p[6usize]] {
                                                            if a[p[4usize]] > a[p[5usize]] {
                                                                p = [
                                                                    p[0usize],
                                                                    p[1usize],
                                                                    p[2usize],
                                                                    p[3usize],
                                                                    p[5usize],
                                                                    p[4usize],
                                                                    p[6usize],
                                                                ];
                                                            }
                                                            if a[p[4usize]] < a[p[6usize]] {
                                                                if a[p[5usize]] > a[p[6usize]] {
                                                                    p = [
                                                                        p[0usize],
                                                                        p[1usize],
                                                                        p[2usize],
                                                                        p[3usize],
                                                                        p[4usize],
                                                                        p[6usize],
                                                                        p[5usize],
                                                                    ];
                                                                }
                                                                return [
                                                                    a[p[0usize]],
                                                                    a[p[1usize]],
                                                                    a[p[2usize]],
                                                                    a[p[3usize]],
                                                                    a[p[4usize]],
                                                                    a[p[5usize]],
                                                                    a[p[6usize]],
                                                                ];
                                                            } else {
                                                                return [
                                                                    a[p[0usize]],
                                                                    a[p[1usize]],
                                                                    a[p[2usize]],
                                                                    a[p[3usize]],
                                                                    a[p[6usize]],
                                                                    a[p[4usize]],
                                                                    a[p[5usize]],
                                                                ];
                                                            }
                                                        } else {
                                                            if a[p[4usize]] > a[p[5usize]] {
                                                                p = [
                                                                    p[0usize],
                                                                    p[1usize],
                                                                    p[2usize],
                                                                    p[3usize],
                                                                    p[5usize],
                                                                    p[4usize],
                                                                    p[6usize],
                                                                ];
                                                            }
                                                            return [
                                                                a[p[0usize]],
                                                                a[p[1usize]],
                                                                a[p[2usize]],
                                                                a[p[6usize]],
                                                                a[p[3usize]],
                                                                a[p[4usize]],
                                                                a[p[5usize]],
                                                            ];
                                                        }
                                                    } else {
                                                        if a[p[3usize]] < a[p[6usize]] {
                                                            if a[p[4usize]] > a[p[6usize]] {
                                                                p = [
                                                                    p[0usize],
                                                                    p[1usize],
                                                                    p[2usize],
                                                                    p[3usize],
                                                                    p[6usize],
                                                                    p[5usize],
                                                                    p[4usize],
                                                                ];
                                                            }
                                                            return [
                                                                a[p[0usize]],
                                                                a[p[1usize]],
                                                                a[p[2usize]],
                                                                a[p[5usize]],
                                                                a[p[3usize]],
                                                                a[p[4usize]],
                                                                a[p[6usize]],
                                                            ];
                                                        } else {
                                                            if a[p[5usize]] > a[p[6usize]] {
                                                                p = [
                                                                    p[0usize],
                                                                    p[1usize],
                                                                    p[2usize],
                                                                    p[3usize],
                                                                    p[4usize],
                                                                    p[6usize],
                                                                    p[5usize],
                                                                ];
                                                            }
                                                            return [
                                                                a[p[0usize]],
                                                                a[p[1usize]],
                                                                a[p[2usize]],
                                                                a[p[5usize]],
                                                                a[p[6usize]],
                                                                a[p[3usize]],
                                                                a[p[4usize]],
                                                            ];
                                                        }
                                                    }
                                                } else {
                                                    if a[p[3usize]] > a[p[4usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[3usize]] < a[p[5usize]] {
                                                        if a[p[4usize]] > a[p[5usize]] {
                                                            p = [
                                                                p[0usize],
                                                                p[1usize],
                                                                p[2usize],
                                                                p[3usize],
                                                                p[5usize],
                                                                p[4usize],
                                                                p[6usize],
                                                            ];
                                                        }
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[6usize]],
                                                            a[p[2usize]],
                                                            a[p[3usize]],
                                                            a[p[4usize]],
                                                            a[p[5usize]],
                                                        ];
                                                    } else {
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[6usize]],
                                                            a[p[2usize]],
                                                            a[p[5usize]],
                                                            a[p[3usize]],
                                                            a[p[4usize]],
                                                        ];
                                                    }
                                                }
                                            } else {
                                                if a[p[2usize]] < a[p[6usize]] {
                                                    if a[p[3usize]] > a[p[4usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[3usize]] < a[p[6usize]] {
                                                        if a[p[4usize]] > a[p[6usize]] {
                                                            p = [
                                                                p[0usize],
                                                                p[1usize],
                                                                p[2usize],
                                                                p[3usize],
                                                                p[6usize],
                                                                p[5usize],
                                                                p[4usize],
                                                            ];
                                                        }
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[5usize]],
                                                            a[p[2usize]],
                                                            a[p[3usize]],
                                                            a[p[4usize]],
                                                            a[p[6usize]],
                                                        ];
                                                    } else {
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[5usize]],
                                                            a[p[2usize]],
                                                            a[p[6usize]],
                                                            a[p[3usize]],
                                                            a[p[4usize]],
                                                        ];
                                                    }
                                                } else {
                                                    if a[p[3usize]] > a[p[4usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[5usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[4usize],
                                                            p[6usize],
                                                            p[5usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[1usize]],
                                                        a[p[5usize]],
                                                        a[p[6usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                    ];
                                                }
                                            }
                                        } else {
                                            if a[p[2usize]] < a[p[5usize]] {
                                                if a[p[2usize]] < a[p[6usize]] {
                                                    if a[p[3usize]] > a[p[5usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[5usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[3usize]] < a[p[6usize]] {
                                                        if a[p[5usize]] > a[p[6usize]] {
                                                            p = [
                                                                p[0usize],
                                                                p[1usize],
                                                                p[2usize],
                                                                p[3usize],
                                                                p[4usize],
                                                                p[6usize],
                                                                p[5usize],
                                                            ];
                                                        }
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[4usize]],
                                                            a[p[2usize]],
                                                            a[p[3usize]],
                                                            a[p[5usize]],
                                                            a[p[6usize]],
                                                        ];
                                                    } else {
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[4usize]],
                                                            a[p[2usize]],
                                                            a[p[6usize]],
                                                            a[p[3usize]],
                                                            a[p[5usize]],
                                                        ];
                                                    }
                                                } else {
                                                    if a[p[3usize]] > a[p[5usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[5usize],
                                                            p[4usize],
                                                            p[3usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[4usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[6usize],
                                                            p[5usize],
                                                            p[4usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[1usize]],
                                                        a[p[4usize]],
                                                        a[p[6usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[5usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[2usize]] < a[p[6usize]] {
                                                    if a[p[3usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[6usize],
                                                            p[4usize],
                                                            p[5usize],
                                                            p[3usize],
                                                        ];
                                                    }
                                                    if a[p[4usize]] > a[p[5usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[4usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[1usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[6usize]],
                                                    ];
                                                } else {
                                                    if a[p[4usize]] > a[p[5usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[4usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    if a[p[4usize]] < a[p[6usize]] {
                                                        if a[p[5usize]] > a[p[6usize]] {
                                                            p = [
                                                                p[0usize],
                                                                p[1usize],
                                                                p[2usize],
                                                                p[3usize],
                                                                p[4usize],
                                                                p[6usize],
                                                                p[5usize],
                                                            ];
                                                        }
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[4usize]],
                                                            a[p[5usize]],
                                                            a[p[6usize]],
                                                            a[p[2usize]],
                                                            a[p[3usize]],
                                                        ];
                                                    } else {
                                                        return [
                                                            a[p[0usize]],
                                                            a[p[1usize]],
                                                            a[p[6usize]],
                                                            a[p[4usize]],
                                                            a[p[5usize]],
                                                            a[p[2usize]],
                                                            a[p[3usize]],
                                                        ];
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[4usize]] {
                                            if a[p[2usize]] < a[p[5usize]] {
                                                if a[p[3usize]] > a[p[4usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                if a[p[3usize]] < a[p[5usize]] {
                                                    if a[p[4usize]] > a[p[5usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[5usize],
                                                            p[4usize],
                                                            p[6usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[6usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                    ];
                                                } else {
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[6usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[5usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[3usize]] > a[p[4usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[6usize]],
                                                    a[p[1usize]],
                                                    a[p[5usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[2usize]] < a[p[5usize]] {
                                                if a[p[3usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[6usize]],
                                                    a[p[1usize]],
                                                    a[p[4usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[5usize]],
                                                ];
                                            } else {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[6usize]],
                                                    a[p[1usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                ];
                                            }
                                        }
                                    }
                                } else {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[4usize]] {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[3usize]] > a[p[4usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                if a[p[3usize]] < a[p[6usize]] {
                                                    if a[p[4usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[6usize],
                                                            p[5usize],
                                                            p[4usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[5usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                        a[p[6usize]],
                                                    ];
                                                } else {
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[5usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[6usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[3usize]] > a[p[4usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[5usize]],
                                                    a[p[1usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[3usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[6usize],
                                                        p[4usize],
                                                        p[5usize],
                                                        p[3usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[5usize]],
                                                    a[p[1usize]],
                                                    a[p[4usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                if a[p[4usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[6usize],
                                                        p[5usize],
                                                        p[4usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[5usize]],
                                                    a[p[1usize]],
                                                    a[p[4usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                ];
                                            }
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[4usize]] {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                            ];
                                        } else {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[4usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[3usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                if a[p[3usize]] < a[p[6usize]] {
                                                    if a[p[5usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[4usize],
                                                            p[6usize],
                                                            p[5usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[4usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[3usize]],
                                                        a[p[5usize]],
                                                        a[p[6usize]],
                                                    ];
                                                } else {
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[4usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[6usize]],
                                                        a[p[3usize]],
                                                        a[p[5usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[3usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[3usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[4usize]],
                                                    a[p[1usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[5usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[3usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[6usize],
                                                        p[4usize],
                                                        p[5usize],
                                                        p[3usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[4usize]],
                                                    a[p[1usize]],
                                                    a[p[5usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[4usize]],
                                                    a[p[1usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                ];
                                            }
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                            ];
                                        } else {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[6usize],
                                                    p[4usize],
                                                    p[5usize],
                                                    p[3usize],
                                                ];
                                            }
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[3usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[3usize],
                                                p[2usize],
                                                p[4usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[4usize]] < a[p[6usize]] {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        } else {
                                            return [
                                                a[p[0usize]],
                                                a[p[6usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[4usize],
                                                p[3usize],
                                                p[2usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                if a[p[4usize]] < a[p[6usize]] {
                                                    if a[p[5usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[4usize],
                                                            p[6usize],
                                                            p[5usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[3usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                        a[p[6usize]],
                                                    ];
                                                } else {
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[3usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                        a[p[6usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[3usize]],
                                                    a[p[1usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[2usize]] < a[p[6usize]] {
                                                if a[p[4usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[6usize],
                                                        p[5usize],
                                                        p[4usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[3usize]],
                                                    a[p[1usize]],
                                                    a[p[5usize]],
                                                    a[p[2usize]],
                                                    a[p[4usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[3usize]],
                                                    a[p[1usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                    a[p[2usize]],
                                                    a[p[4usize]],
                                                ];
                                            }
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[4usize],
                                                p[3usize],
                                                p[2usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[3usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[6usize],
                                                    p[4usize],
                                                    p[5usize],
                                                    p[3usize],
                                                ];
                                            }
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                            ];
                                        } else {
                                            if a[p[3usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[6usize],
                                                    p[4usize],
                                                    p[5usize],
                                                    p[3usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[4usize],
                                                p[3usize],
                                                p[2usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[4usize],
                                                p[3usize],
                                                p[2usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[5usize],
                                                p[4usize],
                                                p[3usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                            ];
                                        } else {
                                            return [
                                                a[p[0usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[5usize],
                                                p[3usize],
                                                p[4usize],
                                                p[2usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[5usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[5usize],
                                                p[3usize],
                                                p[4usize],
                                                p[2usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[5usize]],
                                            ];
                                        } else {
                                            return [
                                                a[p[0usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[5usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[1usize]] < a[p[6usize]] {
                                        if a[p[2usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[6usize],
                                                p[3usize],
                                                p[4usize],
                                                p[5usize],
                                                p[2usize],
                                            ];
                                        }
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] < a[p[5usize]] {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[0usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            return [
                                                a[p[0usize]],
                                                a[p[5usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[1usize]],
                                                a[p[2usize]],
                                                a[p[6usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        if a[p[3usize]] < a[p[5usize]] {
                                            if a[p[3usize]] < a[p[6usize]] {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                if a[p[4usize]] < a[p[6usize]] {
                                                    if a[p[5usize]] > a[p[6usize]] {
                                                        p = [
                                                            p[0usize],
                                                            p[1usize],
                                                            p[2usize],
                                                            p[3usize],
                                                            p[4usize],
                                                            p[6usize],
                                                            p[5usize],
                                                        ];
                                                    }
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[3usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                        a[p[6usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                    ];
                                                } else {
                                                    return [
                                                        a[p[0usize]],
                                                        a[p[3usize]],
                                                        a[p[6usize]],
                                                        a[p[4usize]],
                                                        a[p[5usize]],
                                                        a[p[1usize]],
                                                        a[p[2usize]],
                                                    ];
                                                }
                                            } else {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[6usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[3usize]] < a[p[6usize]] {
                                                if a[p[4usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[6usize],
                                                        p[5usize],
                                                        p[4usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[5usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[6usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                ];
                                            } else {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[0usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                ];
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[4usize]] {
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[3usize]] < a[p[5usize]] {
                                                if a[p[4usize]] > a[p[5usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[5usize],
                                                        p[4usize],
                                                        p[6usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[6usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[6usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[5usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[6usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] < a[p[5usize]] {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[6usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[4usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                            ];
                                        } else {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[6usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[4usize]] {
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[4usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[3usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[5usize],
                                                p[4usize],
                                                p[3usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[1usize]] < a[p[5usize]] {
                                    if a[p[2usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[5usize],
                                            p[3usize],
                                            p[4usize],
                                            p[2usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[5usize]] {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[4usize]] {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[3usize]] < a[p[6usize]] {
                                                if a[p[4usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[6usize],
                                                        p[5usize],
                                                        p[4usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[5usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[5usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[6usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[3usize]] > a[p[4usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[5usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[6usize],
                                                    p[4usize],
                                                    p[5usize],
                                                    p[3usize],
                                                ];
                                            }
                                            return [
                                                a[p[5usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[4usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[5usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[4usize]] {
                                        if a[p[3usize]] > a[p[4usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[4usize],
                                                p[3usize],
                                                p[5usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[4usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[3usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[6usize],
                                                p[4usize],
                                                p[5usize],
                                                p[3usize],
                                            ];
                                        }
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[4usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[6usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[6usize],
                                            p[3usize],
                                            p[4usize],
                                            p[5usize],
                                            p[2usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[4usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[6usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[4usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                    ];
                                } else {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[4usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[4usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[3usize]] < a[p[6usize]] {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[4usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[4usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[6usize]],
                                                    a[p[3usize]],
                                                    a[p[5usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[3usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[3usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[4usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[5usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[3usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[6usize],
                                                    p[4usize],
                                                    p[5usize],
                                                    p[3usize],
                                                ];
                                            }
                                            return [
                                                a[p[4usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[4usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[3usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[3usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[5usize],
                                                p[4usize],
                                                p[3usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[3usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[6usize],
                                                p[4usize],
                                                p[5usize],
                                                p[3usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[3usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[3usize],
                                            p[2usize],
                                            p[4usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[5usize],
                                            p[3usize],
                                            p[4usize],
                                            p[2usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[5usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[5usize],
                                            p[3usize],
                                            p[4usize],
                                            p[2usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[6usize],
                                            p[3usize],
                                            p[4usize],
                                            p[5usize],
                                            p[2usize],
                                        ];
                                    }
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[5usize]] {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[3usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[4usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[3usize],
                                    p[5usize],
                                    p[4usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[4usize]] < a[p[6usize]] {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[4usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[3usize],
                                    p[5usize],
                                    p[4usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[4usize]] < a[p[6usize]] {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[3usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                }
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[4usize]] < a[p[6usize]] {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[3usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[3usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[2usize]],
                                                    a[p[6usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[3usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[2usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[3usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[3usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[2usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[5usize]] {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[4usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[6usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[4usize],
                                            p[3usize],
                                            p[2usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[5usize],
                                            p[3usize],
                                            p[4usize],
                                            p[2usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[2usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[5usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[2usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[5usize],
                                            p[3usize],
                                            p[4usize],
                                            p[2usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[2usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[6usize],
                                            p[3usize],
                                            p[4usize],
                                            p[5usize],
                                            p[2usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[2usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[5usize]] {
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[2usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[5usize],
                                    p[4usize],
                                    p[3usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[5usize],
                                    p[4usize],
                                    p[3usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[2usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[5usize],
                                    p[3usize],
                                    p[4usize],
                                    p[2usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[6usize]] {
                            if a[p[2usize]] > a[p[6usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[6usize],
                                    p[3usize],
                                    p[4usize],
                                    p[5usize],
                                    p[2usize],
                                ];
                            }
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                return [
                                    a[p[5usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                    a[p[6usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[2usize],
                                    p[4usize],
                                    p[3usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[3usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            } else {
                                return [
                                    a[p[5usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[2usize]] {
                            p = [
                                p[0usize],
                                p[2usize],
                                p[1usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[3usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[2usize],
                                p[4usize],
                                p[3usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[3usize]] < a[p[5usize]] {
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] < a[p[6usize]] {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[2usize]],
                                    ];
                                }
                            } else {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        } else {
                            if a[p[3usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            } else {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[2usize]],
                                ];
                            }
                        }
                    }
                }
            }
        }
    } else {
        if a[p[0usize]] < a[p[3usize]] {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[5usize]] {
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[4usize]] < a[p[6usize]] {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[2usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[2usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                    a[p[3usize]],
                                                    a[p[6usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                            ];
                                        } else {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[5usize]] {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[5usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[4usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[6usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[0usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[1usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[1usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[0usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                            a[p[1usize]],
                                            a[p[3usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[2usize]],
                                            a[p[0usize]],
                                            a[p[6usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[1usize]],
                                            a[p[3usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[5usize]] {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[5usize]] {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[2usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[6usize],
                                        p[3usize],
                                        p[4usize],
                                        p[5usize],
                                        p[2usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[6usize]] {
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[2usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[5usize],
                                        p[3usize],
                                        p[4usize],
                                        p[2usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[4usize]] {
                            if a[p[2usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[5usize],
                                    p[3usize],
                                    p[4usize],
                                    p[2usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[5usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[5usize],
                                    p[3usize],
                                    p[4usize],
                                    p[2usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[4usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[6usize]] {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[2usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[4usize],
                                        p[3usize],
                                        p[2usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[6usize]] {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[4usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[4usize],
                                    p[3usize],
                                    p[2usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            } else {
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[3usize],
                                p[2usize],
                                p[1usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[4usize],
                                p[3usize],
                                p[2usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] < a[p[5usize]] {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] < a[p[6usize]] {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[6usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[3usize]],
                                    ];
                                }
                            } else {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            } else {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[3usize]],
                                ];
                            }
                        }
                    }
                }
            }
        } else {
            if a[p[0usize]] < a[p[4usize]] {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[4usize],
                                p[2usize],
                                p[3usize],
                                p[1usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] < a[p[6usize]] {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[1usize]] < a[p[6usize]] {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[2usize]] > a[p[3usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[3usize],
                                        p[2usize],
                                        p[4usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[4usize],
                                p[2usize],
                                p[3usize],
                                p[1usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[5usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[5usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[4usize],
                                p[2usize],
                                p[3usize],
                                p[1usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[6usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                    a[p[6usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[4usize]] {
                            p = [
                                p[0usize],
                                p[4usize],
                                p[2usize],
                                p[3usize],
                                p[1usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] < a[p[5usize]] {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[6usize]] {
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[4usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[6usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[4usize]],
                                    ];
                                }
                            } else {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            } else {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[4usize]],
                                ];
                            }
                        }
                    }
                }
            } else {
                if a[p[0usize]] < a[p[5usize]] {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[5usize],
                                p[2usize],
                                p[3usize],
                                p[4usize],
                                p[1usize],
                                p[6usize],
                            ];
                        }
                        if a[p[1usize]] < a[p[6usize]] {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[4usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[5usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[4usize],
                                        p[6usize],
                                        p[5usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                    a[p[6usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] > a[p[3usize]] {
                                p = [
                                    p[0usize],
                                    p[1usize],
                                    p[3usize],
                                    p[2usize],
                                    p[4usize],
                                    p[5usize],
                                    p[6usize],
                                ];
                            }
                            if a[p[2usize]] < a[p[4usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                return [
                                    a[p[4usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[6usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[1usize]] > a[p[5usize]] {
                            p = [
                                p[0usize],
                                p[5usize],
                                p[2usize],
                                p[3usize],
                                p[4usize],
                                p[1usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] < a[p[4usize]] {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[6usize]] {
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[6usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[5usize]],
                                    ];
                                }
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] < a[p[6usize]] {
                                if a[p[3usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[6usize],
                                        p[4usize],
                                        p[5usize],
                                        p[3usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[6usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                ];
                            } else {
                                if a[p[4usize]] > a[p[6usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[6usize],
                                        p[5usize],
                                        p[4usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[6usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[5usize]],
                                ];
                            }
                        }
                    }
                } else {
                    if a[p[0usize]] < a[p[6usize]] {
                        if a[p[1usize]] > a[p[6usize]] {
                            p = [
                                p[0usize],
                                p[6usize],
                                p[2usize],
                                p[3usize],
                                p[4usize],
                                p[5usize],
                                p[1usize],
                            ];
                        }
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] < a[p[4usize]] {
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                if a[p[3usize]] < a[p[5usize]] {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                    ];
                                } else {
                                    return [
                                        a[p[2usize]],
                                        a[p[5usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                        a[p[6usize]],
                                    ];
                                }
                            } else {
                                if a[p[3usize]] > a[p[4usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[4usize],
                                        p[3usize],
                                        p[5usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[4usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[6usize]],
                                ];
                            }
                        } else {
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[3usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[5usize],
                                        p[4usize],
                                        p[3usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[5usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[6usize]],
                                ];
                            } else {
                                if a[p[4usize]] > a[p[5usize]] {
                                    p = [
                                        p[0usize],
                                        p[1usize],
                                        p[2usize],
                                        p[3usize],
                                        p[5usize],
                                        p[4usize],
                                        p[6usize],
                                    ];
                                }
                                return [
                                    a[p[4usize]],
                                    a[p[5usize]],
                                    a[p[2usize]],
                                    a[p[3usize]],
                                    a[p[0usize]],
                                    a[p[1usize]],
                                    a[p[6usize]],
                                ];
                            }
                        }
                    } else {
                        if a[p[2usize]] > a[p[3usize]] {
                            p = [
                                p[0usize],
                                p[1usize],
                                p[3usize],
                                p[2usize],
                                p[4usize],
                                p[5usize],
                                p[6usize],
                            ];
                        }
                        if a[p[2usize]] < a[p[4usize]] {
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[5usize]] {
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            if a[p[4usize]] < a[p[6usize]] {
                                                if a[p[5usize]] > a[p[6usize]] {
                                                    p = [
                                                        p[0usize],
                                                        p[1usize],
                                                        p[2usize],
                                                        p[3usize],
                                                        p[4usize],
                                                        p[6usize],
                                                        p[5usize],
                                                    ];
                                                }
                                                return [
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[6usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                ];
                                            } else {
                                                return [
                                                    a[p[2usize]],
                                                    a[p[3usize]],
                                                    a[p[6usize]],
                                                    a[p[4usize]],
                                                    a[p[5usize]],
                                                    a[p[0usize]],
                                                    a[p[1usize]],
                                                ];
                                            }
                                        } else {
                                            if a[p[4usize]] > a[p[5usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[5usize],
                                                    p[4usize],
                                                    p[6usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[5usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                            ];
                                        }
                                    } else {
                                        if a[p[3usize]] < a[p[6usize]] {
                                            if a[p[4usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[6usize],
                                                    p[5usize],
                                                    p[4usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[5usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[6usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                            ];
                                        } else {
                                            if a[p[5usize]] > a[p[6usize]] {
                                                p = [
                                                    p[0usize],
                                                    p[1usize],
                                                    p[2usize],
                                                    p[3usize],
                                                    p[4usize],
                                                    p[6usize],
                                                    p[5usize],
                                                ];
                                            }
                                            return [
                                                a[p[2usize]],
                                                a[p[5usize]],
                                                a[p[6usize]],
                                                a[p[3usize]],
                                                a[p[4usize]],
                                                a[p[0usize]],
                                                a[p[1usize]],
                                            ];
                                        }
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[5usize]] {
                                        if a[p[4usize]] > a[p[5usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[5usize],
                                                p[4usize],
                                                p[6usize],
                                            ];
                                        }
                                        return [
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[5usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    }
                                }
                            } else {
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[4usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[6usize],
                                                p[5usize],
                                                p[4usize],
                                            ];
                                        }
                                        return [
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[4usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[4usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[4usize],
                                            p[3usize],
                                            p[5usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[5usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[4usize],
                                            p[6usize],
                                            p[5usize],
                                        ];
                                    }
                                    return [
                                        a[p[5usize]],
                                        a[p[6usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[4usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                    ];
                                }
                            }
                        } else {
                            if a[p[2usize]] < a[p[5usize]] {
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[3usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[4usize]],
                                            a[p[2usize]],
                                            a[p[6usize]],
                                            a[p[3usize]],
                                            a[p[5usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    }
                                } else {
                                    if a[p[3usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[5usize],
                                            p[4usize],
                                            p[3usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[6usize],
                                            p[5usize],
                                            p[4usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[6usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[5usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                    ];
                                }
                            } else {
                                if a[p[2usize]] < a[p[6usize]] {
                                    if a[p[3usize]] > a[p[6usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[6usize],
                                            p[4usize],
                                            p[5usize],
                                            p[3usize],
                                        ];
                                    }
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    return [
                                        a[p[4usize]],
                                        a[p[5usize]],
                                        a[p[2usize]],
                                        a[p[3usize]],
                                        a[p[6usize]],
                                        a[p[0usize]],
                                        a[p[1usize]],
                                    ];
                                } else {
                                    if a[p[4usize]] > a[p[5usize]] {
                                        p = [
                                            p[0usize],
                                            p[1usize],
                                            p[2usize],
                                            p[3usize],
                                            p[5usize],
                                            p[4usize],
                                            p[6usize],
                                        ];
                                    }
                                    if a[p[4usize]] < a[p[6usize]] {
                                        if a[p[5usize]] > a[p[6usize]] {
                                            p = [
                                                p[0usize],
                                                p[1usize],
                                                p[2usize],
                                                p[3usize],
                                                p[4usize],
                                                p[6usize],
                                                p[5usize],
                                            ];
                                        }
                                        return [
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[6usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    } else {
                                        return [
                                            a[p[6usize]],
                                            a[p[4usize]],
                                            a[p[5usize]],
                                            a[p[2usize]],
                                            a[p[3usize]],
                                            a[p[0usize]],
                                            a[p[1usize]],
                                        ];
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
