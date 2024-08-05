#[doc = "Register `FORCEHCLKACTIVE` reader"]
pub type R = crate::R<ForcehclkactiveSpec>;
#[doc = "Register `FORCEHCLKACTIVE` writer"]
pub type W = crate::W<ForcehclkactiveSpec>;
#[doc = "Field `forcehclkactive` reader - 0:0\\]
1 :> Gate HCLK 0 :> UnGate HCLK"]
pub type ForcehclkactiveR = crate::BitReader;
#[doc = "Field `forcehclkactive` writer - 0:0\\]
1 :> Gate HCLK 0 :> UnGate HCLK"]
pub type ForcehclkactiveW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1 :> Gate HCLK 0 :> UnGate HCLK"]
    #[inline(always)]
    pub fn forcehclkactive(&self) -> ForcehclkactiveR {
        ForcehclkactiveR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1 :> Gate HCLK 0 :> UnGate HCLK"]
    #[inline(always)]
    #[must_use]
    pub fn forcehclkactive(&mut self) -> ForcehclkactiveW<ForcehclkactiveSpec> {
        ForcehclkactiveW::new(self, 0)
    }
}
#[doc = "FORCEHCLKACTIVE\n\nYou can [`read`](crate::Reg::read) this register and get [`forcehclkactive::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`forcehclkactive::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForcehclkactiveSpec;
impl crate::RegisterSpec for ForcehclkactiveSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`forcehclkactive::R`](R) reader structure"]
impl crate::Readable for ForcehclkactiveSpec {}
#[doc = "`write(|w| ..)` method takes [`forcehclkactive::W`](W) writer structure"]
impl crate::Writable for ForcehclkactiveSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FORCEHCLKACTIVE to value 0"]
impl crate::Resettable for ForcehclkactiveSpec {
    const RESET_VALUE: u32 = 0;
}
