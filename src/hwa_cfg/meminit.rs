#[doc = "Register `MEMINIT` reader"]
pub type R = crate::R<MeminitSpec>;
#[doc = "Register `MEMINIT` writer"]
pub type W = crate::W<MeminitSpec>;
#[doc = "Field `WIN_RAM_INIT` reader - 0:0\\]
1: Start initialising Window RAM with all '0's"]
pub type WinRamInitR = crate::BitReader;
#[doc = "Field `WIN_RAM_INIT` writer - 0:0\\]
1: Start initialising Window RAM with all '0's"]
pub type WinRamInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARAM_INIT` reader - 1:1\\]
1: Start initialising Parameter set RAM with all '0's"]
pub type ParamInitR = crate::BitReader;
#[doc = "Field `PARAM_INIT` writer - 1:1\\]
1: Start initialising Parameter set RAM with all '0's"]
pub type ParamInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPING_INIT` reader - 2:2\\]
1: Start initialising ACCEL_MEM0 with all '0's"]
pub type IpingInitR = crate::BitReader;
#[doc = "Field `IPING_INIT` writer - 2:2\\]
1: Start initialising ACCEL_MEM0 with all '0's"]
pub type IpingInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPONG_INIT` reader - 3:3\\]
1: Start initialising ACCEL_MEM1 with all '0's"]
pub type IpongInitR = crate::BitReader;
#[doc = "Field `IPONG_INIT` writer - 3:3\\]
1: Start initialising ACCEL_MEM1 with all '0's"]
pub type IpongInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPING_INIT` reader - 4:4\\]
1: Start initialising ACCEL_MEM2 with all '0's"]
pub type OpingInitR = crate::BitReader;
#[doc = "Field `OPING_INIT` writer - 4:4\\]
1: Start initialising ACCEL_MEM2 with all '0's"]
pub type OpingInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPONG_INIT` reader - 5:5\\]
1: Start initialising ACCEL_MEM3 with all '0's"]
pub type OpongInitR = crate::BitReader;
#[doc = "Field `OPONG_INIT` writer - 5:5\\]
1: Start initialising ACCEL_MEM3 with all '0's"]
pub type OpongInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_EVEN_INIT` reader - 6:6\\]
1: Start initialising MEM_COMPRESSION_EVEN_RAM with all '0's"]
pub type McEvenInitR = crate::BitReader;
#[doc = "Field `MC_EVEN_INIT` writer - 6:6\\]
1: Start initialising MEM_COMPRESSION_EVEN_RAM with all '0's"]
pub type McEvenInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MC_ODD_INIT` reader - 7:7\\]
1: Start initialising MEM_COMPRESSION_ODD_RAM with all '0's"]
pub type McOddInitR = crate::BitReader;
#[doc = "Field `MC_ODD_INIT` writer - 7:7\\]
1: Start initialising MEM_COMPRESSION_ODD_RAM with all '0's"]
pub type McOddInitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU` reader - "]
pub type NuR = crate::FieldReader<u32>;
#[doc = "Field `NU` writer - "]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
1: Start initialising Window RAM with all '0's"]
    #[inline(always)]
    pub fn win_ram_init(&self) -> WinRamInitR {
        WinRamInitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Start initialising Parameter set RAM with all '0's"]
    #[inline(always)]
    pub fn param_init(&self) -> ParamInitR {
        ParamInitR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Start initialising ACCEL_MEM0 with all '0's"]
    #[inline(always)]
    pub fn iping_init(&self) -> IpingInitR {
        IpingInitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Start initialising ACCEL_MEM1 with all '0's"]
    #[inline(always)]
    pub fn ipong_init(&self) -> IpongInitR {
        IpongInitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Start initialising ACCEL_MEM2 with all '0's"]
    #[inline(always)]
    pub fn oping_init(&self) -> OpingInitR {
        OpingInitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Start initialising ACCEL_MEM3 with all '0's"]
    #[inline(always)]
    pub fn opong_init(&self) -> OpongInitR {
        OpongInitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Start initialising MEM_COMPRESSION_EVEN_RAM with all '0's"]
    #[inline(always)]
    pub fn mc_even_init(&self) -> McEvenInitR {
        McEvenInitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Start initialising MEM_COMPRESSION_ODD_RAM with all '0's"]
    #[inline(always)]
    pub fn mc_odd_init(&self) -> McOddInitR {
        McOddInitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
1: Start initialising Window RAM with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn win_ram_init(&mut self) -> WinRamInitW<MeminitSpec> {
        WinRamInitW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
1: Start initialising Parameter set RAM with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn param_init(&mut self) -> ParamInitW<MeminitSpec> {
        ParamInitW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
1: Start initialising ACCEL_MEM0 with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn iping_init(&mut self) -> IpingInitW<MeminitSpec> {
        IpingInitW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
1: Start initialising ACCEL_MEM1 with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn ipong_init(&mut self) -> IpongInitW<MeminitSpec> {
        IpongInitW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
1: Start initialising ACCEL_MEM2 with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn oping_init(&mut self) -> OpingInitW<MeminitSpec> {
        OpingInitW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
1: Start initialising ACCEL_MEM3 with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn opong_init(&mut self) -> OpongInitW<MeminitSpec> {
        OpongInitW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
1: Start initialising MEM_COMPRESSION_EVEN_RAM with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn mc_even_init(&mut self) -> McEvenInitW<MeminitSpec> {
        McEvenInitW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
1: Start initialising MEM_COMPRESSION_ODD_RAM with all '0's"]
    #[inline(always)]
    #[must_use]
    pub fn mc_odd_init(&mut self) -> McOddInitW<MeminitSpec> {
        McOddInitW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<MeminitSpec> {
        NuW::new(self, 8)
    }
}
#[doc = "MEMINIT\n\nYou can [`read`](crate::Reg::read) this register and get [`meminit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meminit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeminitSpec;
impl crate::RegisterSpec for MeminitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meminit::R`](R) reader structure"]
impl crate::Readable for MeminitSpec {}
#[doc = "`write(|w| ..)` method takes [`meminit::W`](W) writer structure"]
impl crate::Writable for MeminitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMINIT to value 0"]
impl crate::Resettable for MeminitSpec {
    const RESET_VALUE: u32 = 0;
}
