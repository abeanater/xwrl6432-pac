#[doc = "Register `MEMINITDONE` reader"]
pub type R = crate::R<MeminitdoneSpec>;
#[doc = "Register `MEMINITDONE` writer"]
pub type W = crate::W<MeminitdoneSpec>;
#[doc = "Field `WIN_RAM_INITDONE` reader - 0:0\\]
1: Init done status for Window RAM"]
pub type WinRamInitdoneR = crate::BitReader;
#[doc = "Field `WIN_RAM_INITDONE` writer - 0:0\\]
1: Init done status for Window RAM"]
pub type WinRamInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARAM_INITDONE` reader - 1:1\\]
1: Init done status for Parameter set RAM"]
pub type ParamInitdoneR = crate::BitReader;
#[doc = "Field `PARAM_INITDONE` writer - 1:1\\]
1: Init done status for Parameter set RAM"]
pub type ParamInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPING_INITDONE` reader - 2:2\\]
1: Init done status for ACCEL_MEM0"]
pub type IpingInitdoneR = crate::BitReader;
#[doc = "Field `IPING_INITDONE` writer - 2:2\\]
1: Init done status for ACCEL_MEM0"]
pub type IpingInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPONG_INITDONE` reader - 3:3\\]
1: Init done status for ACCEL_MEM1"]
pub type IpongInitdoneR = crate::BitReader;
#[doc = "Field `IPONG_INITDONE` writer - 3:3\\]
1: Init done status for ACCEL_MEM1"]
pub type IpongInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPING_INITDONE` reader - 4:4\\]
1: Init done status for ACCEL_MEM2"]
pub type OpingInitdoneR = crate::BitReader;
#[doc = "Field `OPING_INITDONE` writer - 4:4\\]
1: Init done status for ACCEL_MEM2"]
pub type OpingInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPONG_INITDONE` reader - 5:5\\]
1: Init done status for ACCEL_MEM3"]
pub type OpongInitdoneR = crate::BitReader;
#[doc = "Field `OPONG_INITDONE` writer - 5:5\\]
1: Init done status for ACCEL_MEM3"]
pub type OpongInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_EVEN_INITDONE` reader - 6:6\\]
1: Init done status for MEM_COMPRESSION_EVEN_RAM"]
pub type McEvenInitdoneR = crate::BitReader;
#[doc = "Field `MC_EVEN_INITDONE` writer - 6:6\\]
1: Init done status for MEM_COMPRESSION_EVEN_RAM"]
pub type McEvenInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_ODD_INITDONE` reader - 7:7\\]
1: Init done status for MEM_COMPRESSION_ODD_RAM"]
pub type McOddInitdoneR = crate::BitReader;
#[doc = "Field `MC_ODD_INITDONE` writer - 7:7\\]
1: Init done status for MEM_COMPRESSION_ODD_RAM"]
pub type McOddInitdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Init done status for Window RAM"]
    #[inline(always)]
    pub fn win_ram_initdone(&self) -> WinRamInitdoneR {
        WinRamInitdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Init done status for Parameter set RAM"]
    #[inline(always)]
    pub fn param_initdone(&self) -> ParamInitdoneR {
        ParamInitdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Init done status for ACCEL_MEM0"]
    #[inline(always)]
    pub fn iping_initdone(&self) -> IpingInitdoneR {
        IpingInitdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Init done status for ACCEL_MEM1"]
    #[inline(always)]
    pub fn ipong_initdone(&self) -> IpongInitdoneR {
        IpongInitdoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Init done status for ACCEL_MEM2"]
    #[inline(always)]
    pub fn oping_initdone(&self) -> OpingInitdoneR {
        OpingInitdoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Init done status for ACCEL_MEM3"]
    #[inline(always)]
    pub fn opong_initdone(&self) -> OpongInitdoneR {
        OpongInitdoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Init done status for MEM_COMPRESSION_EVEN_RAM"]
    #[inline(always)]
    pub fn mc_even_initdone(&self) -> McEvenInitdoneR {
        McEvenInitdoneR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Init done status for MEM_COMPRESSION_ODD_RAM"]
    #[inline(always)]
    pub fn mc_odd_initdone(&self) -> McOddInitdoneR {
        McOddInitdoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Init done status for Window RAM"]
    #[inline(always)]
    #[must_use]
    pub fn win_ram_initdone(&mut self) -> WinRamInitdoneW<MeminitdoneSpec> {
        WinRamInitdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Init done status for Parameter set RAM"]
    #[inline(always)]
    #[must_use]
    pub fn param_initdone(&mut self) -> ParamInitdoneW<MeminitdoneSpec> {
        ParamInitdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Init done status for ACCEL_MEM0"]
    #[inline(always)]
    #[must_use]
    pub fn iping_initdone(&mut self) -> IpingInitdoneW<MeminitdoneSpec> {
        IpingInitdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Init done status for ACCEL_MEM1"]
    #[inline(always)]
    #[must_use]
    pub fn ipong_initdone(&mut self) -> IpongInitdoneW<MeminitdoneSpec> {
        IpongInitdoneW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Init done status for ACCEL_MEM2"]
    #[inline(always)]
    #[must_use]
    pub fn oping_initdone(&mut self) -> OpingInitdoneW<MeminitdoneSpec> {
        OpingInitdoneW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Init done status for ACCEL_MEM3"]
    #[inline(always)]
    #[must_use]
    pub fn opong_initdone(&mut self) -> OpongInitdoneW<MeminitdoneSpec> {
        OpongInitdoneW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Init done status for MEM_COMPRESSION_EVEN_RAM"]
    #[inline(always)]
    #[must_use]
    pub fn mc_even_initdone(&mut self) -> McEvenInitdoneW<MeminitdoneSpec> {
        McEvenInitdoneW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Init done status for MEM_COMPRESSION_ODD_RAM"]
    #[inline(always)]
    #[must_use]
    pub fn mc_odd_initdone(&mut self) -> McOddInitdoneW<MeminitdoneSpec> {
        McOddInitdoneW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<MeminitdoneSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "MEMINITDONE\n\nYou can [`read`](crate::Reg::read) this register and get [`meminitdone::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meminitdone::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeminitdoneSpec;
impl crate::RegisterSpec for MeminitdoneSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meminitdone::R`](R) reader structure"]
impl crate::Readable for MeminitdoneSpec {}
#[doc = "`write(|w| ..)` method takes [`meminitdone::W`](W) writer structure"]
impl crate::Writable for MeminitdoneSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMINITDONE to value 0"]
impl crate::Resettable for MeminitdoneSpec {
    const RESET_VALUE: u32 = 0;
}
