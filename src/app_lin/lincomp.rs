#[doc = "Register `LINCOMP` reader"]
pub type R = crate::R<LincompSpec>;
#[doc = "Register `LINCOMP` writer"]
pub type W = crate::W<LincompSpec>;
#[doc = "Field `SBREAK` reader - 2:0\\]
3-bit Sync Break extend. LIN mode only. These bits are used to configure the number of Tbits for the sync break to extend the minimum 13 Tbit in the Sync Field to a maximum of 20 Tbit. The time delay calculation for the sync break is: TSYNBRK = 13Tbit + SBREAK x Tbit These bits are writable in LIN mode only."]
pub type SbreakR = crate::FieldReader;
#[doc = "Field `SBREAK` writer - 2:0\\]
3-bit Sync Break extend. LIN mode only. These bits are used to configure the number of Tbits for the sync break to extend the minimum 13 Tbit in the Sync Field to a maximum of 20 Tbit. The time delay calculation for the sync break is: TSYNBRK = 13Tbit + SBREAK x Tbit These bits are writable in LIN mode only."]
pub type SbreakW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDEL` reader - 9:8\\]
2-bit Sync Delimiter compare. These bits are effective in LIN mode only. These bits are used to configure the number of Tbit for the sync delimiter in the sync field. The time delay calculation for the synchronization delimiter is: TSDEL = (SDEL + 1)Tbit These bits are writable in LIN mode only."]
pub type SdelR = crate::FieldReader;
#[doc = "Field `SDEL` writer - 9:8\\]
2-bit Sync Delimiter compare. These bits are effective in LIN mode only. These bits are used to configure the number of Tbit for the sync delimiter in the sync field. The time delay calculation for the synchronization delimiter is: TSDEL = (SDEL + 1)Tbit These bits are writable in LIN mode only."]
pub type SdelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - 15:10\\]
Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 15:10\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - 31:16\\]
Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` writer - 31:16\\]
Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
3-bit Sync Break extend. LIN mode only. These bits are used to configure the number of Tbits for the sync break to extend the minimum 13 Tbit in the Sync Field to a maximum of 20 Tbit. The time delay calculation for the sync break is: TSYNBRK = 13Tbit + SBREAK x Tbit These bits are writable in LIN mode only."]
    #[inline(always)]
    pub fn sbreak(&self) -> SbreakR {
        SbreakR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - 9:8\\]
2-bit Sync Delimiter compare. These bits are effective in LIN mode only. These bits are used to configure the number of Tbit for the sync delimiter in the sync field. The time delay calculation for the synchronization delimiter is: TSDEL = (SDEL + 1)Tbit These bits are writable in LIN mode only."]
    #[inline(always)]
    pub fn sdel(&self) -> SdelR {
        SdelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
3-bit Sync Break extend. LIN mode only. These bits are used to configure the number of Tbits for the sync break to extend the minimum 13 Tbit in the Sync Field to a maximum of 20 Tbit. The time delay calculation for the sync break is: TSYNBRK = 13Tbit + SBREAK x Tbit These bits are writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn sbreak(&mut self) -> SbreakW<LincompSpec> {
        SbreakW::new(self, 0)
    }
    #[doc = "Bits 8:9 - 9:8\\]
2-bit Sync Delimiter compare. These bits are effective in LIN mode only. These bits are used to configure the number of Tbit for the sync delimiter in the sync field. The time delay calculation for the synchronization delimiter is: TSDEL = (SDEL + 1)Tbit These bits are writable in LIN mode only."]
    #[inline(always)]
    #[must_use]
    pub fn sdel(&mut self) -> SdelW<LincompSpec> {
        SdelW::new(self, 8)
    }
    #[doc = "Bits 10:15 - 15:10\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<LincompSpec> {
        Reserved1W::new(self, 10)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved2(&mut self) -> Reserved2W<LincompSpec> {
        Reserved2W::new(self, 16)
    }
}
#[doc = "The LINCOMPARE register is used to configure the sync delimeter and sync break extension.\n\nYou can [`read`](crate::Reg::read) this register and get [`lincomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LincompSpec;
impl crate::RegisterSpec for LincompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lincomp::R`](R) reader structure"]
impl crate::Readable for LincompSpec {}
#[doc = "`write(|w| ..)` method takes [`lincomp::W`](W) writer structure"]
impl crate::Writable for LincompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINCOMP to value 0"]
impl crate::Resettable for LincompSpec {
    const RESET_VALUE: u32 = 0;
}
